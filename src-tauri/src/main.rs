
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager, SystemTray, SystemTrayEvent};
mod tray;

fn main() {
  let system_tray: SystemTray = tray::get_tray();

  tauri::Builder::default()
    .system_tray(system_tray)
    .setup(|app| {

        let main_window = app.get_window("main").unwrap();

        // follow workspace on Mac OS X -> NOTE: this is not working in production
        #[cfg(target_os = "macos")]
        {
            use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior};
            use cocoa::base::id;
            let ns_win = main_window.ns_window().unwrap() as id;
            unsafe {
                ns_win.setLevel_(((NSMainMenuWindowLevel + 1) as u64).try_into().unwrap());
                ns_win.setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces);
                // ns_win.setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
            }
        }

        // remove icon from dock
         app.set_activation_policy(tauri::ActivationPolicy::Accessory);

        // Dev Tool
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          main_window.open_devtools();
          main_window.close_devtools();
        }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick { .. } => {
            let _item_handle = app.tray_handle().get_item("show");
            let window = app.get_window("main").unwrap();
            if window.is_visible().expect("true") {
              _item_handle.set_title("Hide").unwrap();
            } else {
              _item_handle.set_title("Show").unwrap();
            }
      }  
      SystemTrayEvent::MenuItemClick { id, .. } => {
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
        _ => {}
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

