use serde_json::Value;
use serde::Serialize;

#[derive(Serialize)]
pub struct SerializableActiveWindow {
    pub title: String,
    pub app_name: String,
}

#[tauri::command]
pub fn get_active_window_info() -> Result<SerializableActiveWindow, String> {
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => Ok(SerializableActiveWindow {
            title: active_window.title,
            app_name: active_window.app_name,
        }),
        Err(_) => Err("Failed to get active window information".into()),
    }
}
