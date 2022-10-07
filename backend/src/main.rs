#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

mod api;

fn main() {
    tauri::Builder::default()
        .setup(api::bus_setup)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
