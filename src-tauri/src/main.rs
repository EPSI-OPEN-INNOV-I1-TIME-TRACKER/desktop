#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod state;
mod system_tray;

use crate::commands::{get_active_window_info, get_app_usage, get_current_window_time};
use crate::state::app_usage_data::{AppUsageData, SharedAppUsageData};
use crate::system_tray::create_system_tray;
use std::sync::{Arc, Mutex};
use tauri::{generate_context, generate_handler, Builder, Manager};

fn main() {
    let app_usage_data: SharedAppUsageData = Arc::new(Mutex::new(AppUsageData::default()));
    let system_tray = create_system_tray();

    Builder::default()
        .manage(app_usage_data)
        .invoke_handler(generate_handler![
            get_active_window_info,
            get_app_usage,
            get_current_window_time
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
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
            },
            _ => {}
        })
        .setup(|_app| Ok(()))
        .run(generate_context!())
        .expect("error while running tauri application");
}
