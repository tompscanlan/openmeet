// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::env;
mod event;
mod commands;


fn main() {
    env_logger::init();
    log::info!("Starting Tauri application");
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
        commands::greet,
        commands::create_event,
        commands::get_event,
        commands::list_events,
        commands::create_database,
        ])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
