// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use reqwest::Client;
use reqwest::Url;
use crate::event::Event;
use cassandra_cpp::Cluster;
use cassandra_cpp::AsRustType;
use cassandra_cpp::LendingIterator;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
pub async fn create_database(db_name: String) -> Result<String, String> {
    let mut clusterdefault = Cluster::default();
    let cluster = clusterdefault.set_contact_points("127.0.0.1").unwrap();
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    // Create keyspace if it doesn't exist
    let create_keyspace = format!("CREATE KEYSPACE IF NOT EXISTS {} WITH REPLICATION = {{ 'class' : 'SimpleStrategy', 'replication_factor' : 1 }};", db_name);
    let create_keyspace_ref = create_keyspace.clone();
    session.execute(&create_keyspace_ref).await.map_err(|e| e.to_string())?;
    
    Ok(format!("Keyspace '{}' created successfully", db_name))
}

pub fn create_client_with_referer(couchdb_url: &str) -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    let host = Url::parse(couchdb_url)
        .map(|url| url.host_str().unwrap_or("localhost").to_string())
        .unwrap_or("localhost".to_string());
    headers.insert(reqwest::header::REFERER, format!("http://{}", host).parse().unwrap());
    Client::builder().default_headers(headers).build().unwrap()
}

#[tauri::command]
pub async fn create_event(id: String, title: String, description: String, date: String, location: String) -> Result<Event, String> {
    let event = Event { id, title: title.clone(), description: description.clone(), date: date.clone(), location: location.clone() };
    let mut clusterdefault = Cluster::default();
    let cluster = clusterdefault.set_contact_points("127.0.0.1").unwrap();
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method
    let db_name = "events";

    // Create table if it doesn't exist
    let create_table = format!("CREATE TABLE IF NOT EXISTS {} (id UUID PRIMARY KEY, title text, description text, date text, location text);", db_name);
    session.execute(&create_table).await.map_err(|e| e.to_string())?; // Await the execute method

    // Insert event into Cassandra
    let insert_event = format!("INSERT INTO {} (id, title, description, date, location) VALUES (uuid(), '{}', '{}', '{}', '{}');", db_name, title, description, date, location);
    session.execute(&insert_event).await.map_err(|e| e.to_string())?; // Await the execute method

    Ok(event)
}
#[tauri::command]
pub async fn get_event(id: u64) -> Result<Option<Event>, String> {
    let mut clusterdefault = Cluster::default();
    let cluster = clusterdefault.set_contact_points("127.0.0.1").unwrap();
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method
    let db_name = "events";

    let query = format!("SELECT * FROM {} WHERE id = {};", db_name, id);
    let result = session.execute(&query).await.map_err(|e| e.to_string())?; // Await the execute method

    if let Some(row) = result.first_row() {
        let event = Event {
            id: row.get_by_name::<String>("id".to_string()).unwrap(),
            title: row.get_by_name::<String>("title".to_string()).unwrap(),
            description: row.get_by_name::<String>("description".to_string()).unwrap(),
            date: row.get_by_name::<String>("date".to_string()).unwrap(),
            location: row.get_by_name::<String>("location".to_string()).unwrap(),
        };
        Ok(Some(event))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn list_events() -> Result<Vec<Event>, String> {
    let mut clusterdefault = Cluster::default();
    let cluster = clusterdefault.set_contact_points("127.0.0.1").unwrap();
    let session = cluster.connect().await.map_err(|e| e.to_string())?;
    let db_name = "events";

    let query = format!("SELECT * FROM {};", db_name);
    let result = session.execute(&query).await.map_err(|e| e.to_string())?;

    let mut events = Vec::new();
    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        let event = Event {
            id: row.get_column_by_name::<String>("id".to_string()).unwrap().to_string(),
            title: row.get_column_by_name::<String>("title".to_string()).unwrap().to_string(),
            description: row.get_column_by_name::<String>("description".to_string()).unwrap().to_string(),
            date: row.get_column_by_name::<String>("date".to_string()).unwrap().to_string(),
            location: row.get_column_by_name::<String>("location".to_string()).unwrap().to_string(),
        };
        events.push(event);
    }

    Ok(events)
}
