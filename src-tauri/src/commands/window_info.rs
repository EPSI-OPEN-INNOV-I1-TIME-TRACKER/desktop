use serde::Serialize;
use tauri::command;

#[derive(Serialize)]
pub struct SerializableActiveWindow {
    pub title: String,
    pub app_name: String,
}

#[command]
pub fn get_active_window_info() -> Result<SerializableActiveWindow, String> {    // Hypothetical call to the fictional library `active_win_pos_rs`
    // This library is supposed to provide the active window's title and app name.
    // Since this is a fictional example, replace this with your actual library call.
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => Ok(SerializableActiveWindow {
            title: active_window.title,
            app_name: active_window.app_name,
        }),
        Err(e) => Err(format!("Failed to get active window information: {:?}", e)),
    }
}
