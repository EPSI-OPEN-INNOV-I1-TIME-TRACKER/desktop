use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem};

pub fn create_system_tray() -> SystemTray {
    // Define custom menu items
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Show");

    // Create the system tray menu
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)  // Add the "Quit" option
        .add_native_item(SystemTrayMenuItem::Separator)  // Add a separator
        .add_item(hide)  // Add the "Hide" option
        .add_item(show); // Add the "Show" option

    // Create the system tray with the menu
    let system_tray = SystemTray::new().with_menu(tray_menu);

    // Return the configured system tray
    system_tray
}
