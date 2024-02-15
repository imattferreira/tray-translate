use std::process;

#[tauri::command]
pub fn quit() {
    process::exit(0)
}
