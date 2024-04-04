
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::SystemTray;

mod tray;
mod system_tray_events;
mod setup;

fn main() {
  let system_tray: SystemTray = tray::get_tray();

  tauri::Builder::default()
    .system_tray(system_tray)
    .setup(|app| setup::setup(app))
    .invoke_handler(tauri::generate_handler![])
    .on_system_tray_event(|app, event| system_tray_events::handle_system_tray_event(app, event))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

