mod app;
mod ui;
mod ws;
mod commands;

use tauri::Builder;

fn main() {
    Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::get_active_window_info
        ])
        .setup(|_app| {
            // Setup initial application state or perform startup tasks
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
