use tauri::{App, Manager};
use std::error::Error;
use tauri_nspanel::{panel_delegate, ManagerExt, WindowExt};


pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let main_window = app.get_window("main").ok_or("Main window not found")?;

        #[cfg(target_os = "macos")]
    {
        let panel = main_window.to_panel().unwrap();

        let delegate = panel_delegate!(MyPanelDelegate {
            window_did_become_key,
            window_did_resign_key
        });

        let handle = main_window.app_handle();
        delegate.set_listener(Box::new(move |delegate_name: String| {
            match delegate_name.as_str() {
            "window_did_become_key" => {
                let app_name = handle.package_info().name.to_owned();
                println!("[info]: {:?} panel becomes key window!", app_name);
            }
            "window_did_resign_key" => {
                println!("[info]: panel resigned from key window!");
            }
            _ => (),
            }
        }));

        panel.set_delegate(delegate);
    }

    // // follow workspace on Mac OS X -> NOTE: this is not working in production
    // #[cfg(target_os = "macos")]
    // {
    //     use cocoa::appkit::{NSMainMenuWindowLevel, NSWindow, NSWindowCollectionBehavior};
    //     use cocoa::base::id;
    //     let ns_win = main_window.ns_window().unwrap() as id;
    //     unsafe {
    //         ns_win.setLevel_(((NSMainMenuWindowLevel + 1) as u64).try_into().unwrap());
    //         ns_win.setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces);
    //         // ns_win.setCollectionBehavior_(NSWindowCollectionBehavior::NSWindowCollectionBehaviorMoveToActiveSpace);
    //     }
    // }

    // remove icon from dock
    app.set_activation_policy(tauri::ActivationPolicy::Accessory);

    // Dev Tool
    #[cfg(debug_assertions)] // only include this code on debug builds
    {
        main_window.open_devtools();
        main_window.close_devtools();
    }

    Ok(())
}
