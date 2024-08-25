// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use reqwest::Client;
use reqwest::Url;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
struct Event {
    id: u64,
    title: String,
    description: String,
    date: String,
    location: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    env_logger::init();
    log::info!("Starting Tauri application");
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, create_event, get_event, list_events, create_database])
    .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn create_database(dbName: String) -> Result<String, String> {
    let client = Client::new();
    let couchdb_url = env::var("COUCHDB_URL").unwrap_or("http://localhost:5984".to_string());
    let db_url = format!("{}/{}", couchdb_url, dbName);

    log::info!("CouchDB URL: {}", couchdb_url);
log::info!("Database URL: {}", db_url);
    log::info!("Creating CouchDB database at URL: {}", db_url);

    let res = client.put(&db_url)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                Ok(format!("Database '{}' created successfully", dbName))
            } else {
                let error_text = response.text().await.unwrap_or("Unknown error".to_string());
                log::error!("Failed to create database: {}", error_text);
                Err(error_text)
            }
        },
        Err(e) => {
            log::error!("Request error: {}", e);
            Err(e.to_string())
        }
    }
}

fn create_client_with_referer(couchdb_url: &str) -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    let host = Url::parse(couchdb_url)
        .map(|url| url.host_str().unwrap_or("localhost").to_string())
        .unwrap_or("localhost".to_string());
    headers.insert(reqwest::header::REFERER, format!("http://{}", host).parse().unwrap());
    Client::builder().default_headers(headers).build().unwrap()
}

#[tauri::command]
async fn create_event(id: u64, title: String, description: String, date: String, location: String) -> Result<Event, String> {
    let event = Event { id, title: title.clone(), description: description.clone(), date: date.clone(), location: location.clone() };
    let couchdb_url = env::var("COUCHDB_URL").unwrap_or("http://localhost:5984".to_string());
    let db_name = "events";
    let db_url = format!("{}/{}", couchdb_url, db_name);

    let client = create_client_with_referer(&couchdb_url);
    let db_exists = client.head(&db_url).send().await.map_err(|e| e.to_string())?.status().is_success();
    if !db_exists {
        let _ = create_database(db_name.to_string()).await;
    }

    // Create JSON payload
    let payload = serde_json::json!({
        "id": id,
        "title": title,
        "description": description,
        "date": date,
        "location": location
    });

    let res = client.post(&db_url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                Ok(event)
            } else {
                let error_text = response.text().await.unwrap_or("Unknown error".to_string());
                log::error!("Failed to create event: {}", error_text);
                Err(error_text)
            }
        },
        Err(e) => {
            log::error!("Request error: {}", e);
            Err(e.to_string())
        }
    }
}
#[tauri::command]
async fn get_event(id: u64) -> Result<Option<Event>, String> {
    let client = Client::new();
    let couchdb_url = env::var("COUCHDB_URL").unwrap_or("http://localhost:5984/events".to_string());

    let res = client.get(&format!("{}/{}", couchdb_url, id))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let event = res.json::<Event>().await.map_err(|e| e.to_string())?;
        Ok(Some(event))
    } else if res.status().as_u16() == 404 {
        Ok(None)
    } else {
        Err(res.text().await.unwrap_or("Unknown error".to_string()))
    }
}

#[tauri::command]
async fn list_events() -> Result<Vec<Event>, String> {
    let couchdb_url = env::var("COUCHDB_URL").unwrap_or("http://localhost:5984".to_string());
    let db_name = "events";
    let view_url = format!("{}/{}/_design/events/_view/all", couchdb_url, db_name);

    let client = create_client_with_referer(&couchdb_url);
    
    let res = client.get(&view_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        let rows = body["rows"].as_array().ok_or("Invalid response format")?;
        
        let events: Vec<Event> = rows.iter()
            .filter_map(|row| {
                let doc = row["value"].as_object()?;
                Some(Event {
                    id: doc["id"].as_u64()?,
                    title: doc["title"].as_str()?.to_string(),
                    description: doc["description"].as_str()?.to_string(),
                    date: doc["date"].as_str()?.to_string(),
                    location: doc["location"].as_str()?.to_string(),
                })
            })
            .collect();

        Ok(events)
    } else {
        Err(format!("Failed to fetch events: {}", res.status()))
    }
}