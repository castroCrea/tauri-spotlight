
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Manager};

fn main() {
  tauri::Builder::default()
    .setup(|app| {

        let main_window = app.get_window("main").unwrap();

        // follow workspace on Mac OS X
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

        // Dev Tool
        #[cfg(debug_assertions)] // only include this code on debug builds
        {
          main_window.open_devtools();
          main_window.close_devtools();
        }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
    println!("Message from Rust: {}", name);

   format!("Hello, {}!", name)
}
