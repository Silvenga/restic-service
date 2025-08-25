use tauri::{AppHandle, Manager, WebviewWindow};
use tauri_plugin_positioner::{Position, WindowExt};

const STATUS_WINDOW_ID: &str = "status";

pub fn toggle_status_window_visibility(app_handle: &AppHandle) {
    let window = get_status_window(app_handle);
    if window.is_visible().unwrap() {
        close_status_window(app_handle);
    } else {
        show_status_window(app_handle);
    }
}

pub fn show_status_window(app_handle: &AppHandle) {
    let window = get_status_window(app_handle);
    let _ = window.move_window_constrained(Position::TrayRight);
    let _ = window.show();
    let _ = window.unminimize();
    let _ = window.set_focus();
}

pub fn close_status_window(app_handle: &AppHandle) {
    let window = get_status_window(app_handle);
    let _ = window.minimize();
    let _ = window.hide();
}

pub fn get_status_window(app_handle: &AppHandle) -> WebviewWindow {
    app_handle
        .get_webview_window(STATUS_WINDOW_ID)
        .expect("status window should always exists")
}
