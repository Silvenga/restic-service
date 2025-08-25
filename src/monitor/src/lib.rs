mod tray_icon;
mod windows;

use crate::tray_icon::setup_tray;
use crate::windows::show_status_window;
use tauri::App;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            show_status_window(app);
        }))
        .plugin(tauri_plugin_positioner::init())
        .setup(|app: &mut App| {
            setup_tray(app)?;
            Ok(())
        })
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
