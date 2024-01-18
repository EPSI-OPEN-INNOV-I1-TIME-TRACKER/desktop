#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{generate_context, generate_handler, Builder};

// Structure intermédiaire sérialisable
#[derive(Serialize)]
struct SerializableActiveWindow {
    title: String,
    app_name: String,
    // Ajoutez ici d'autres champs si nécessaire
}

#[tauri::command]
fn get_active_window_info() -> Result<SerializableActiveWindow, String> {
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => Ok(SerializableActiveWindow {
            title: active_window.title,
            app_name: active_window.app_name,
            // Initialisez ici d'autres champs si nécessaire
        }),
        Err(_) => Err("Failed to get active window information".into()),
    }
}

fn main() {
    Builder::default()
        .invoke_handler(generate_handler![get_active_window_info])
        .run(generate_context!())
        .expect("error while running tauri application");
}
