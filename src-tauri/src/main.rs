#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, WindowEvent, generate_context, generate_handler, Builder, State};
use tauri::SystemTrayMenuItem;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tauri::async_runtime::spawn;
use tokio::time::sleep as async_sleep;
use rusqlite::{Connection, Result as SqlResult};
use warp::Filter;

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
    time_spent: HashMap<String, Duration>,
}

impl Default for AppUsageData {
    fn default() -> Self {
        Self {
            last_active: Instant::now(),
            time_spent: HashMap::new(),
        }
    }
}

type SharedAppUsageData = Arc<Mutex<AppUsageData>>;

#[tauri::command]
fn get_app_usage(data: State<'_, SharedAppUsageData>) -> HashMap<String, Duration> {
    data.lock().unwrap().time_spent.clone()
}

#[tauri::command]
fn get_current_window_time(data: State<'_, SharedAppUsageData>) -> Result<Duration, String> {
    let data = data.lock().unwrap();
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => {
            data.time_spent.get(&active_window.app_name)
                .cloned()
                .ok_or_else(|| "No data available for the current window".to_string())
        },
        Err(_) => Err("Failed to get active window information".into()),
    }
}

fn connect_to_db() -> SqlResult<Connection> {
    Connection::open("window_tracker.db")
}

fn create_tables(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS window_usage (
            id INTEGER PRIMARY KEY,
            window_title TEXT NOT NULL,
            app_name TEXT NOT NULL,
            duration INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )", []
    )?;
    Ok(())
}

async fn run_websocket_server() {
    let websocket_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| {
                async move {
                    use warp::ws::{Message, WebSocket};

                    let (mut tx, mut rx) = websocket.split();
                    while let Some(result) = rx.next().await {
                        match result {
                            Ok(msg) => {
                                if msg.is_text() {
                                    tx.send(Message::text("Echo: " + msg.to_str().unwrap())).await.unwrap();
                                }
                            },
                            Err(e) => {
                                eprintln!("websocket error: {:?}", e);
                                break;
                            }
                        }
                    }
                }
            })
        });

    warp::serve(websocket_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

fn main() {
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

    Builder::default()
        .manage(app_usage_data.clone())
        .invoke_handler(generate_handler![get_active_window_info, get_app_usage, get_current_window_time])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    },
                    "hide" => {
                        if let Some(window) = app.get_window("main") {
                            window.hide().unwrap();
                        }
                    },
                    "show" => {
                        if let Some(window) = app.get_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    },
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
            let conn = connect_to_db().expect("Failed to connect to database");
            create_tables(&conn).expect("Failed to create tables");

            spawn(async move {
                let mut last_app_name = String::new();
                loop {
                    async_sleep(Duration::from_secs(1)).await;
                    match get_active_window_info() {
                        Ok(active_window) => {
                            let mut data = app_usage_data_clone.lock().unwrap();
                            if last_app_name != active_window.app_name {
                                data.last_active = Instant::now();
                                last_app_name = active_window.app_name.clone();
                            }
                            let last_active_clone = data.last_active;
                            let entry = data.time_spent.entry(active_window.app_name).or_insert_with(Duration::default);
                            *entry += Instant::now().duration_since(last_active_clone);
                            data.last_active = Instant::now();
                        },
                        Err(_) => {}
                    }
                }
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");

    // Before running the Tauri Builder
    std::thread::spawn(|| {
        tokio::runtime::Runtime::new().unwrap().block_on(run_websocket_server());
    });
}
