use tauri::{AppHandle, Manager, SystemTray, SystemTrayEvent};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn make_tray() -> SystemTray {
    SystemTray::new()
}

fn toggle_window(app: &AppHandle) {
    let window = app.get_window("main").unwrap();
    let _ = window.move_window(Position::TrayCenter);

    if window.is_visible().unwrap() {
        window.hide().unwrap();
        return;
    }

    window.show().unwrap();
    window.set_focus().unwrap();
}

pub fn handle_tray_events(app: &AppHandle, event: SystemTrayEvent) {
    tauri_plugin_positioner::on_tray_event(app, &event);

    if let SystemTrayEvent::LeftClick { .. } = event {
        toggle_window(app)
    }
}
