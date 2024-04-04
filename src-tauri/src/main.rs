
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::SystemTray;
use std::process::Command;

mod tray;
mod system_tray_events;
mod setup;

fn main() {
  let system_tray: SystemTray = tray::get_tray();

  tauri::Builder::default()
    .system_tray(system_tray)
    .setup(|app| setup::setup(app))
    .invoke_handler(tauri::generate_handler![execute_jxa_script])
    .on_system_tray_event(|app, event| system_tray_events::handle_system_tray_event(app, event))
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


fn execute_jxa(script: &str) -> Result<String, std::io::Error> {
    let output = Command::new("osascript")
        .arg("-l")
        .arg("JavaScript")
        .arg("-e")
        .arg(script)
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
fn execute_jxa_script(script: String) -> Result<String, String> {
    match execute_jxa(&script) {
        Ok(output) => Ok(output),
        Err(err) => Err(format!("Error executing AppleScript: {}", err)),
    }
}
