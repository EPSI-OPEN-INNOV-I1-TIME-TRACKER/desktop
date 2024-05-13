#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, Manager, WindowEvent, generate_context, generate_handler, Builder};
use tauri::SystemTrayMenuItem;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tauri::async_runtime::spawn;
use tokio::time::sleep as async_sleep;
use rusqlite::{Connection, params, Result as SqlResult};

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

fn connect_to_db() -> SqlResult<Connection> {
    Connection::open("window_tracker.db")
}

fn create_tables(conn: &Connection) -> SqlResult<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS window_usage (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            window_title TEXT NOT NULL,
            app_name TEXT NOT NULL,
            duration INTEGER NOT NULL,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )", [],
    )?;
    Ok(())
}

fn update_usage(conn: &Connection, app_name: &str, duration: i64) -> SqlResult<()> {
    conn.execute(
        "INSERT INTO window_usage (window_title, app_name, duration, timestamp) VALUES (?, ?, ?, CURRENT_TIMESTAMP)",
        params![app_name, app_name, duration],
    )?;
    Ok(())
}

fn main() {
    let conn = Arc::new(Mutex::new(connect_to_db().expect("Failed to connect to database")));
    create_tables(&conn.lock().unwrap()).expect("Failed to create tables");

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
        .setup(move |_app| {
            let app_usage_data_clone = app_usage_data.clone();
            let conn_clone = conn.clone();
            spawn(async move {
                let mut last_app_name = String::new();
                loop {
                    async_sleep(Duration::from_secs(1)).await;
                    if let Ok(active_window) = get_active_window_info() {
                        let mut data = app_usage_data_clone.lock().unwrap();
                        if last_app_name != active_window.app_name {
                            last_app_name = active_window.app_name.clone();
                            data.last_active = Instant::now();
                        }
                        let last_active_clone = data.last_active;
                        let entry = data.time_spent.entry(active_window.app_name.clone()).or_insert_with(Duration::default);
                        *entry += Instant::now().duration_since(last_active_clone);

                        data.last_active = Instant::now();
                        let duration = data.time_spent.get(&active_window.app_name).unwrap().as_secs() as i64;
                        let conn = conn_clone.lock().unwrap();
                        update_usage(&conn, &active_window.app_name, duration).expect("Failed to update database");
                    }
                }
            });
            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}
