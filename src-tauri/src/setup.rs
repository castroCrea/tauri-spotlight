use tauri::{App, Manager};
use std::error::Error;

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;

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
    // app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Dev Tool
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
        main_window.open_devtools();
        main_window.close_devtools();
    }

    Ok(())
}
