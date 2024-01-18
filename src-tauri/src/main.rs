#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{generate_context, generate_handler, Builder};
use std::{cell::RefCell, time::Instant};

thread_local! {
    static WINDOW_TIME_TRACKER: RefCell<WindowTimeTracker> = RefCell::new(WindowTimeTracker::new());
}

struct WindowTimeTracker {
    start_time: Option<Instant>,
    total_time: u64, // Temps total en secondes
}

impl WindowTimeTracker {
    fn new() -> Self {
        WindowTimeTracker {
            start_time: None,
            total_time: 0,
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs();
            self.total_time += elapsed;
            self.start_time = None;
        }
    }

    fn get_total_time(&self) -> u64 {
        self.total_time
    }
}


#[derive(Serialize)]
struct SerializableActiveWindow {
    title: String,
    app_name: String,
    active_time: u64, // Temps passé sur la fenêtre
    // Autres champs...
}

#[tauri::command]
fn get_active_window_info() -> Result<SerializableActiveWindow, String> {
    match active_win_pos_rs::get_active_window() {
        Ok(active_window) => {
            WINDOW_TIME_TRACKER.with(|tracker| {
                let mut tracker = tracker.borrow_mut();
                tracker.start();
                Ok(SerializableActiveWindow { // Enveloppez dans Ok()
                    title: active_window.title,
                    app_name: active_window.app_name,
                    active_time: tracker.get_total_time(),
                    // Initialisez les autres champs...
                })
            })
        },
        Err(_) => Err("Failed to get active window information".into()),
    }
}



fn main() {
    Builder::default()
        .invoke_handler(generate_handler![get_active_window_info])
        .run(generate_context!())
        .expect("error while running tauri application");
}
