#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use anyhow::Error;
// use tauri_plugin_log::{LoggerBuilder, LogTarget};

fn main() -> Result<(), Error> {
    // env_logger::try_init()?;
    tauri::Builder::default()
        .setup(backend::bus_setup)
        /*
        .plugin(LoggerBuilder::default().targets([
            // LogTarget::LogDir,
            LogTarget::Stdout,
            // LogTarget::Webview,
        ]).build())
        */
        .run(tauri::generate_context!())?;
    Ok(())
}
