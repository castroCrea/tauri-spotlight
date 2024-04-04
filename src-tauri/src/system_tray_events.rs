use tauri::{AppHandle, Manager, SystemTrayEvent};

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => handle_left_click(app),
        SystemTrayEvent::MenuItemClick { id, .. } => handle_menu_item_click(app, id),
        _ => {}
    }
}

fn handle_left_click(app: &AppHandle) {
    let _item_handle = app.tray_handle().get_item("show");
    let window = app.get_window("main").unwrap();
    if window.is_visible().expect("true") {
        _item_handle.set_title("Hide").unwrap();
    } else {
        _item_handle.set_title("Show").unwrap();
    }
}

fn handle_menu_item_click(app: &AppHandle, id: String) {
    match id.as_str() {
        "show" => {
            let window = app.get_window("main").unwrap();
            if window.is_visible().expect("true") {
                window.hide().unwrap();
            } else {
                window.show().unwrap();
            }
        }
        "quit" => {
            app.exit(0)
        }
        _ => {}
    }
}
