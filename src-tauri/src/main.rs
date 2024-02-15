// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod tray;

use commands::quit;
use tray::{handle_tray_events, make_tray};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .invoke_handler(tauri::generate_handler![quit])
        .system_tray(make_tray())
        .on_system_tray_event(handle_tray_events)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
