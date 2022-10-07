#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use anyhow::Error;

fn main() -> Result<(), Error> {
    tauri::Builder::default()
        .setup(backend::bus_setup)
        .run(tauri::generate_context!())?;
    Ok(())
}
