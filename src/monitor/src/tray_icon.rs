use crate::windows::toggle_status_window_visibility;
use log::warn;
use tauri::App;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, TrayIconBuilder, TrayIconEvent};

const QUIT_ID: &str = "quit";

pub fn setup_tray(app: &mut App) -> tauri::Result<()> {
    let quit_item = MenuItem::with_id(app, QUIT_ID, "&Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&quit_item])?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            QUIT_ID => {
                app.exit(0);
            }
            _ => {
                warn!("Menu item {:?} not handled.", event.id);
            }
        })
        .on_tray_icon_event(move |tray, event| {
            let app_handle = tray.app_handle();
            match event {
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => {
                    toggle_status_window_visibility(app_handle);
                }
                _ => {
                    warn!("Tray icon event {event:?} not handled.");
                }
            }

            tauri_plugin_positioner::on_tray_event(app_handle, &event);
        })
        .build(app)?;

    Ok(())
}
