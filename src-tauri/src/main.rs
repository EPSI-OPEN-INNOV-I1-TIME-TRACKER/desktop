#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tauri::{generate_context, generate_handler, Builder};
use std::{cell::RefCell, time::Instant};


thread_local! {
    static WINDOW_TIME_TRACKER: RefCell<WindowManager> = RefCell::new(WindowManager::new());
}

struct WindowManager {
    last_check: Instant,
    last_active_window: Option<String>,
    // ... autres champs si nécessaire ...
}

impl WindowManager {
    fn new() -> WindowManager {
        WindowManager {
            start_time: None,
            total_time: Duration::new(0, 0),
        }
    }

    fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    fn update(&mut self) {
        let now = Instant::now();
        match active_win_pos_rs::get_active_window() {
            Ok(active_window) => {
                let current_window = active_window.title; // ou `app_name`, selon vos besoins
                if self.last_active_window.as_ref() == Some(&current_window) {
                    // La même fenêtre est toujours active
                    let time_spent = now.duration_since(self.last_check).as_secs();
                    // Ajoutez `time_spent` au compteur total pour cette fenêtre
                } else {
                    // La fenêtre active a changé
                    // Traitez le temps passé sur l'ancienne fenêtre
                    // Réinitialisez le compteur pour la nouvelle fenêtre
                    self.last_active_window = Some(current_window);
                }
            },
            Err(_) => {
                // Gérez les erreurs, par exemple si la fenêtre active ne peut pas être obtenue
            }
        }
        self.last_check = now;
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
            let active_time = WINDOW_TIME_TRACKER.with(|manager| {
                let mut manager = manager.borrow_mut();
                manager.update(); // Met à jour le gestionnaire de temps
                manager.calculate_active_time(&active_window.title) // Calcule le temps passé
            });

            Ok(SerializableActiveWindow {
                title: active_window.title,
                app_name: active_window.app_name,
                active_time, // Utilisez la valeur calculée ici
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
