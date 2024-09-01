// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
mod commands;
mod event;
mod user;
use cassandra_cpp::Cluster;

fn main() {
    env_logger::init();
    log::info!("Starting Tauri application");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::greet,
            // commands::create_event,
            // commands::get_event,
            // commands::list_events,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub async fn init_cluster() -> Result<Cluster, String> {
    let mut cluster = Cluster::default();
    let contact_points = env::var("CASSANDRA_CONTACT_POINTS")
        .map_err(|_| "CASSANDRA_CONTACT_POINTS environment variable not set".to_string())?;
    cluster.set_contact_points(&contact_points)
        .map_err(|e| format!("Failed to set contact points: {}", e))?;
    
    let username = env::var("CASSANDRA_USERNAME").unwrap_or_default();
    let password = env::var("CASSANDRA_PASSWORD").unwrap_or_default();

    if !username.is_empty() && !password.is_empty() {
        cluster.set_credentials(&username, &password)
            .map_err(|e| format!("Failed to set credentials: {}", e))?;
    }
    
    Ok(cluster)
}
