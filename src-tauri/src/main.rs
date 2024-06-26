#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod app;
mod ui;
mod ws;
mod commands;

use tauri::{Builder, Manager, WindowEvent, SystemTrayEvent};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep as async_sleep;
use crate::ws::server::update_time_spent;

fn main() {
    let db_conn = Arc::new(Mutex::new(app::db::connect_to_db().expect("Failed to connect to database")));
    let app_usage_data = Arc::new(Mutex::new(app::data::AppUsageData::default()));

    let quit = tauri::CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = tauri::CustomMenuItem::new("hide".to_string(), "Hide");
    let show = tauri::CustomMenuItem::new("show".to_string(), "Show");
    let tray_menu = tauri::SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(tauri::SystemTrayMenuItem::Separator)
        .add_item(hide)
        .add_item(show);
    let system_tray = tauri::SystemTray::new().with_menu(tray_menu);

    let addr = "127.0.0.1:3030".parse().unwrap();

    Builder::default()
        .manage(app_usage_data.clone())
        .invoke_handler(tauri::generate_handler![
            commands::get_active_window_info,
            commands::get_tracked_apps
        ])
        .system_tray(system_tray)
        .on_system_tray_event(move |app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
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
            },
            _ => {}
        })
        .on_window_event(|event| match event.event() {
            WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            },
            _ => {}
        })
        .setup(move |app| {
            let app_handle = app.app_handle();
            let conn_clone1 = db_conn.clone();
            tauri::async_runtime::spawn(async move {
                ws::server::run_websocket_server(addr, app_handle, conn_clone1).await;
            });

            let app_usage_data_clone = app_usage_data.clone();
            let conn_clone2 = db_conn.clone();
            tauri::async_runtime::spawn(async move {
                let mut last_app_name = String::new();
                loop {
                    async_sleep(Duration::from_secs(1)).await;
                    if let Ok(active_window) = commands::get_active_window_info() {
                        let mut data = app_usage_data_clone.lock().unwrap();
                        let current_time = Instant::now();
                        let elapsed_time = current_time.duration_since(data.last_active);
                
                        // Update last active time
                        data.last_active = current_time;
                
                        // If there's already an entry for the app, update its duration
                        if let Some(entry) = data.time_spent.get_mut(&active_window.app_name) {
                            entry.1 += elapsed_time;
                        } else {
                            // No entry exists, create one
                            let window_id = app::db::update_or_insert_window(&conn_clone2.lock().unwrap(), &active_window.title, &active_window.app_name).unwrap();
                            data.time_spent.insert(active_window.app_name.clone(), (window_id, elapsed_time));
                        }
                
                        last_app_name = active_window.app_name.clone();
                    }
                }                
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
