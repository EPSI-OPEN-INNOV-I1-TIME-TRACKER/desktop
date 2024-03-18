use crate::state::app_usage_data::SharedAppUsageData;
use crate::commands::window_info::get_active_window_info;
use std::collections::HashMap;
use std::time::Duration;
use tauri::State;

#[tauri::command]
pub fn get_app_usage(data: State<'_, SharedAppUsageData>) -> HashMap<String, Duration> {
    data.lock().unwrap().time_spent.clone()
}

#[tauri::command]
pub fn get_current_window_time(data: State<'_, SharedAppUsageData>) -> Result<Duration, String> {
    let data = data.lock().unwrap();

    // Assuming get_active_window_info() returns the current active window's app name.
    match get_active_window_info() {
        Ok(active_window) => {
            // Look up the time spent on the active window's app name.
            match data.time_spent.get(&active_window.app_name) {
                Some(duration) => Ok(*duration),
                None => Err("No data available for the current window".into()),
            }
        },
        Err(e) => Err(format!("Failed to get active window information: {}", e)),
    }
}
