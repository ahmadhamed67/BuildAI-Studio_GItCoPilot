#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod utils;

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            commands::file_handler::read_file,
            commands::file_handler::write_file,
            commands::file_handler::create_dir,
            commands::system_info::get_system_info,
            commands::system_info::get_app_version,
            commands::window_control::minimize_window,
            commands::window_control::maximize_window,
            commands::window_control::close_window,
        ])
        .setup(|app| {
            // Open dev tools in debug mode
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }

            println!("Application started successfully!");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
