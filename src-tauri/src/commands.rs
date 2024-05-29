use serde::Serialize;
use std::sync::{Arc, Mutex};
use tauri::State;

use crate::app::data::AppUsageData;

#[derive(Serialize)]
pub struct SerializableActiveWindow {
    pub title: String,
    pub app_name: String,
}

#[derive(Serialize)]
pub struct SerializableAppUsage {
    pub app_name: String,
    pub window_id: i64,
    pub duration: u64, // Duration in seconds
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

#[tauri::command]
pub fn get_tracked_apps(
    state: State<'_, Arc<Mutex<AppUsageData>>>,
) -> Result<Vec<SerializableAppUsage>, String> {
    let data = state
        .lock()
        .map_err(|_| "Failed to lock app usage data".to_string())?;

    let result: Vec<SerializableAppUsage> = data
        .time_spent
        .iter()
        .map(|(app_name, &(window_id, duration))| SerializableAppUsage {
            app_name: app_name.clone(),
            window_id,
            duration: duration.as_secs(),
        })
        .collect();

    Ok(result)
}
