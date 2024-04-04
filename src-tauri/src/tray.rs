use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem };

pub fn get_tray() -> SystemTray {
    let show = CustomMenuItem::new("show".to_string(), "Show").accelerator("Alt+Space");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
    .add_item(show)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

    return SystemTray::new().with_menu(tray_menu);
}
