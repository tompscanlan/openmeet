// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use crate::event::Event;
use cassandra_cpp::AsRustType;
use cassandra_cpp::Cluster;
use cassandra_cpp::LendingIterator;

pub async fn init_cluster() -> Cluster {
    let mut cluster = Cluster::default();
    cluster.set_contact_points("couchdb1.scanlanservices.com").unwrap();
    cluster.set_credentials("tscanlan", "butterball").unwrap();
    cluster
}

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
pub async fn create_event(
    id: String,
    title: String,
    description: String,
    date: String,
    location: String,
) -> Result<Event, String> {
    let mut cluster = init_cluster().await;
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method

    let event = Event {
        id,
        title: title.clone(),
        description: description.clone(),
        date: date.clone(),
        location: location.clone(),
    };
    let db_name = "events";

    // Create table if it doesn't exist
    let create_table = format!("CREATE TABLE IF NOT EXISTS {} (id UUID PRIMARY KEY, title text, description text, date text, location text);", db_name);
    session
        .execute(&create_table)
        .await
        .map_err(|e| e.to_string())?; // Await the execute method

    // Insert event into Cassandra
    let insert_event = format!("INSERT INTO {} (id, title, description, date, location) VALUES (uuid(), '{}', '{}', '{}', '{}');", db_name, title, description, date, location);
    session
        .execute(&insert_event)
        .await
        .map_err(|e| e.to_string())?; // Await the execute method

    Ok(event)
}

#[tauri::command]
pub async fn get_event(id: u64) -> Result<Option<Event>, String> {
    let mut clusterdefault = Cluster::default();
    let cluster = clusterdefault.set_contact_points("127.0.0.1").unwrap();
    cluster
        .set_contact_points("couchdb1.scanlanservices.com")
        .unwrap();

    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method
    let db_name = "events";

    let query = format!("SELECT * FROM {} WHERE id = {};", db_name, id);
    let result = session.execute(&query).await.map_err(|e| e.to_string())?; // Await the execute method

    if let Some(row) = result.first_row() {
        let event = Event {
            id: row.get_by_name::<String>("id".to_string()).unwrap(),
            title: row.get_by_name::<String>("title".to_string()).unwrap(),
            description: row
                .get_by_name::<String>("description".to_string())
                .unwrap(),
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
    let mut cluster = init_cluster().await;
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method
    let db_name = "events";

    // Use the events keyspace
    session
        .execute("USE events")
        .await
        .map_err(|e| e.to_string())?;

    let query = format!("SELECT * FROM {};", db_name);
    let result = session.execute(&query).await.map_err(|e| e.to_string())?;

    let mut events = Vec::new();
    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        let event = Event {
            id: row
                .get_column_by_name::<String>("id".to_string())
                .unwrap()
                .to_string(),
            title: row
                .get_column_by_name::<String>("title".to_string())
                .unwrap()
                .to_string(),
            description: row
                .get_column_by_name::<String>("description".to_string())
                .unwrap()
                .to_string(),
            date: row
                .get_column_by_name::<String>("date".to_string())
                .unwrap()
                .to_string(),
            location: row
                .get_column_by_name::<String>("location".to_string())
                .unwrap()
                .to_string(),
        };
        events.push(event);
    }

    Ok(events)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // disable test
    #[tokio::test]
    #[ignore]
    async fn test_list_events() {
        let _cluster = init_cluster().await;
        let events = list_events().await.unwrap();
        assert!(events.len() == 0);
    }

    #[tokio::test]
    #[ignore]
    async fn test_connect() {
        let mut cluster = Cluster::default();
        cluster
            .set_contact_points("couchdb1.scanlanservices.com")
            .unwrap();

        let session = cluster.connect().await.unwrap();
        println!("{:?}", session);
    }
     
}
