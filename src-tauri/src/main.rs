#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde_json::Value;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, WindowEvent, generate_context, generate_handler, Builder, AppHandle, Wry};
use tauri::SystemTrayMenuItem;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use futures_util::StreamExt;
use tauri::async_runtime::spawn;
use tokio::time::sleep as async_sleep;
use rusqlite::{Connection, OptionalExtension, params, Result as SqlResult};
use serde::Serialize;
use warp::Filter;
use warp::ws::WebSocket;

#[derive(Serialize)]
struct SerializableActiveWindow {
    title: String,
    app_name: String,
}

#[tauri::command]
fn get_active_window_info() -> Result<SerializableActiveWindow, String> {
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => Ok(SerializableActiveWindow {
            title: active_window.title,
            app_name: active_window.app_name,
        }),
        Err(_) => Err("Failed to get active window information".into()),
    }
}

struct AppUsageData {
    last_active: Instant,
    time_spent: HashMap<String, (i64, Duration)>, // Mapping app_name to (window_id, Duration)
}

impl Default for AppUsageData {
    fn default() -> Self {
        Self {
            last_active: Instant::now(),
            time_spent: HashMap::new(),
        }
    }
}

fn connect_to_db() -> SqlResult<Connection> {
    let conn = Connection::open("window_tracker.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS windows (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            window_title TEXT NOT NULL,
            app_name TEXT NOT NULL
        )", [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS time (
            window_id INTEGER NOT NULL,
            duration INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (window_id) REFERENCES windows (id)
        )", [],
    )?;
    Ok(conn)
}

fn update_or_insert_window(conn: &Connection, title: &str, app_name: &str) -> SqlResult<i64> {
    let mut stmt = conn.prepare("SELECT id FROM windows WHERE app_name = ?")?;
    if let Some(result) = stmt.query_row(params![app_name], |row| row.get(0)).optional()? {
        Ok(result)
    } else {
        conn.execute(
            "INSERT INTO windows (window_title, app_name) VALUES (?, ?)",
            params![title, app_name],
        )?;
        Ok(conn.last_insert_rowid())
    }
}

async fn run_websocket_server(addr: SocketAddr, app_handle: AppHandle<Wry>, conn: Arc<Mutex<Connection>>) {
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::any().map(move || app_handle.clone()))
        .and(warp::any().map(move || conn.clone()))
        .map(|ws: warp::ws::Ws, app_handle: AppHandle<Wry>, conn: Arc<Mutex<Connection>>| {
            ws.on_upgrade(move |websocket| handle_connection(websocket, app_handle, conn))
        });

    warp::serve(websocket_route).run(addr).await;
}

async fn handle_connection(websocket: WebSocket, _app_handle: AppHandle, conn: Arc<Mutex<Connection>>) {
    let (_tx, mut rx) = websocket.split();  // Correct unused and mutable unnecessary issues

    while let Some(result) = rx.next().await {
        match result {
            Ok(msg) => {
                if msg.is_text() {
                    let data = msg.to_str().unwrap();
                    println!("Received message: {}", data);

                    match serde_json::from_str::<Value>(data) {
                        Ok(parsed) => {
                            if let (Some(app_name), Some(duration)) = (
                                parsed["app_name"].as_str(),
                                parsed["duration"].as_i64(),
                            ) {
                                let db_conn = conn.lock().unwrap();
                                match update_time_spent(&db_conn, app_name.parse().unwrap(), duration) {
                                    Ok(_) => println!("Database updated successfully"),
                                    Err(e) => eprintln!("Error updating database: {}", e),
                                }
                            }
                        }
                        Err(e) => eprintln!("Error parsing JSON: {}", e),
                    }
                }
                if msg.is_close() {
                    break;
                }
            }
            Err(e) => eprintln!("websocket error: {:?}", e),
        }
    }
}

fn update_time_spent(conn: &Connection, window_id: i64, duration: i64) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO time (window_id, duration) VALUES (?, ?)",
        params![window_id, duration],
    )?;

    Ok(())
}

fn main() {
    let conn = Arc::new(Mutex::new(connect_to_db().expect("Failed to connect to database")));

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");

    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show);

    let system_tray = SystemTray::new().with_menu(tray_menu);

    let app_usage_data = Arc::new(Mutex::new(AppUsageData::default()));

    let addr = "127.0.0.1:3030".parse().unwrap();

    Builder::default()
        .manage(app_usage_data.clone())
        .invoke_handler(generate_handler![get_active_window_info])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    "hide" => {
                        if let Some(window) = app.get_window("main") {
                            window.hide().unwrap();
                        }
                    }
                    "show" => {
                        if let Some(window) = app.get_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .setup(move |app| {
            let app_usage_data_clone = app_usage_data.clone();
            let conn_clone = conn.clone();
            spawn(async move {
                let mut last_app_name = String::new();
                loop {
                    async_sleep(Duration::from_secs(1)).await;
                    if let Ok(active_window) = get_active_window_info() {
                        let mut data = app_usage_data_clone.lock().unwrap();

                        // Check if the app name has changed
                        let app_name_changed = last_app_name != active_window.app_name;

                        // Get current data without keeping a mutable borrow
                        let last_active = data.last_active;
                        let window_id: i64;
                        let mut duration_spent = Duration::default();

                        if let Some(entry) = data.time_spent.get_mut(&active_window.app_name) {
                            // Update the duration if the app is the same
                            duration_spent = entry.1 + Instant::now().duration_since(last_active);
                            window_id = entry.0;
                        } else {
                            // Insert a new entry if the app name has changed
                            last_app_name = active_window.app_name.clone();
                            window_id = update_or_insert_window(&conn_clone.lock().unwrap(), &active_window.title, &active_window.app_name).unwrap();
                            data.time_spent.insert(active_window.app_name.clone(), (window_id, Duration::default()));
                        }

                        // Now we can update last_active since we're not holding a reference to entry anymore
                        if app_name_changed {
                            data.last_active = Instant::now();
                        }

                        // Perform database update outside the data borrowing scope
                        if app_name_changed && duration_spent.as_secs() > 0 {
                            update_time_spent(&conn_clone.lock().unwrap(), window_id, duration_spent.as_secs() as i64)
                                .expect("Failed to update database");
                            // Reset duration after updating
                            if let Some(entry) = data.time_spent.get_mut(&active_window.app_name) {
                                entry.1 = Duration::default();
                            }
                        }
                    }
                }
            });

            // Start WebSocket server
            let app_handle = app.handle().clone();
            let conn_clone = conn.clone(); // Ensure you clone `conn` before moving it into the async block
            spawn(async move {
                run_websocket_server(addr, app_handle, conn_clone).await;
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
