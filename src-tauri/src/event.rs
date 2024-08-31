use std::fmt::Debug;
use cassandra_cpp::UuidGen;
use cassandra_cpp::Uuid;
use cassandra_cpp::*;
use crate::init_cluster;
use chrono::Utc;

// -- Events table
// CREATE TABLE events (
//   event_id UUID,
//   creator_id UUID,
//   title TEXT,
//   description TEXT,
//   start_time TIMESTAMP,
//   end_time TIMESTAMP,
//   lat DOUBLE,
//   lon DOUBLE,
//   address TEXT,
//   created_at TIMESTAMP,
//   updated_at TIMESTAMP,
//   PRIMARY KEY ((creator_id), start_time, event_id)
// ) WITH CLUSTERING ORDER BY (start_time DESC, event_id ASC);
#[derive(Debug)]
pub struct Event {
    pub event_id: Uuid,
    pub creator_id: Uuid,
    pub title: String,
    pub description: String,
    pub start_time: i64,
    pub end_time: i64,
    pub lat: f64,
    pub lon: f64,
    pub address: String,
    pub created_at: i64,
    pub updated_at: i64,
}


#[tauri::command]
pub async fn create_event(
    creator_id: Uuid,
    title: String,
    description: String,
    start_time: i64,
    end_time: i64,
    lat: f64,
    lon: f64,
    address: String
) -> Result<Event> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;
    
    let timestamp_gen = TimestampGen::gen_monotonic_new();
    let uuid_gen = UuidGen::default();

    let event_id = uuid_gen.gen_time();
    let now = Utc::now().timestamp_micros();

    let event = Event {
        event_id,
        creator_id,
        title,
        description,
        start_time,
        end_time,
        lat,
        lon,
        address,
        created_at: now,
        updated_at: now,
    };

    let db_name = "openmeet.events";

    // Insert event into Cassandra
    let insert_event = format!(
        "INSERT INTO {} (event_id, creator_id, title, description, start_time, end_time, lat, lon, address, created_at, updated_at) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        db_name
    );
    let values = (
        event.event_id,
        event.creator_id,
        &event.title,
        &event.description,
        event.start_time,
        event.end_time,
        event.lat,
        event.lon,
        &event.address,
        event.created_at,
        event.updated_at,
    );

    let mut session_statement = session.statement(insert_event);
    session_statement.bind(0, event.event_id).map_err(|e| e.to_string())?;
    session_statement.bind(1, event.creator_id).map_err(|e| e.to_string())?;
    session_statement.bind(2, event.title.as_str()).map_err(|e| e.to_string())?;
    session_statement.bind(3, event.description.as_str()).map_err(|e| e.to_string())?;
    session_statement.bind(4, event.start_time).map_err(|e| e.to_string())?;
    session_statement.bind(5, event.end_time).map_err(|e| e.to_string())?;
    session_statement.bind(6, event.lat).map_err(|e| e.to_string())?;
    session_statement.bind(7, event.lon).map_err(|e| e.to_string())?;
    session_statement.bind(8, event.address.as_str()).map_err(|e| e.to_string())?;
    session_statement.bind(9, event.created_at).map_err(|e| e.to_string())?;
    session_statement.bind(10, event.updated_at).map_err(|e| e.to_string())?;

    session_statement.execute().await.map_err(|e| e.to_string())?;

    Ok(event)
}

#[tauri::command]
pub async fn get_event(creator_id: Uuid, start_time: i64, event_id: Uuid) -> Result<Option<Event>> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;
    let db_name = "openmeet.events";
    println!("getting event creator_id: {:?}, start_time: {:?}, event_id: {:?}", creator_id, start_time, event_id);

    // Prepare the query to select the event by creator_id, start_time, and event_id
    let query = format!("SELECT * FROM {} WHERE creator_id = ? AND start_time = ? AND event_id = ?;", db_name);
    let mut statement = session.statement(query.clone());
    
    statement.bind_by_name("creator_id", creator_id)?;
    statement.bind_by_name("start_time", start_time)?;
    statement.bind_by_name("event_id", event_id)?;

    // Execute the query
    let result = statement.execute().await.map_err(|e| e.to_string())?;

    // Check if we have a result and map it to an Event
    if let Some(row) = result.first_row() {
        let event = Event {
            event_id: row.get_column_by_name("event_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            creator_id: row.get_column_by_name("creator_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            title: row.get_column_by_name("title").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            description: row.get_column_by_name("description").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            start_time: row.get_column_by_name("start_time").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            end_time: row.get_column_by_name("end_time").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            lat: row.get_column_by_name("lat").and_then(|v| v.get_f64()).map_err(|e| e.to_string())?,
            lon: row.get_column_by_name("lon").and_then(|v| v.get_f64()).map_err(|e| e.to_string())?,
            address: row.get_column_by_name("address").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            created_at: row.get_column_by_name("created_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            updated_at: row.get_column_by_name("updated_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
        };
        Ok(Some(event))
    } else {
        Ok(None) // No event found
    }
}

#[tauri::command]
pub async fn list_events() -> Result<Vec<Event>> {

    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method
    let db_name = "openmeet.events";

    let query = format!("SELECT * FROM {};", db_name);
    let result = session.execute(&query).await.map_err(|e| e.to_string())?;

    let mut events = Vec::new();

    let mut iterator = result.iter();
    while let Some(row) = iterator.next() {
                let event = Event {
            event_id: row.get_column_by_name("event_id")
                .and_then(|v| v.get_uuid())
                .map_err(|e| e.to_string())?,
            creator_id: row.get_column_by_name("creator_id")
                .and_then(|v| v.get_uuid())
                .map_err(|e| e.to_string())?,
            title: row.get_column_by_name("title")
                .and_then(|v| v.get_string())
                .map_err(|e| e.to_string())?,
            description: row.get_column_by_name("description")
                .and_then(|v| v.get_string())
                .map_err(|e| e.to_string())?,
            start_time: row.get_column_by_name("start_time")
                .and_then(|v| v.get_i64())
                .map_err(|e| e.to_string())?,
            end_time: row.get_column_by_name("end_time")
                .and_then(|v| v.get_i64())
                .map_err(|e| e.to_string())?,
            lat: row.get_column_by_name("lat")
                .and_then(|v| v.get_f64())
                .map_err(|e| e.to_string())?,
            lon: row.get_column_by_name("lon")
                .and_then(|v| v.get_f64())
                .map_err(|e| e.to_string())?,
            address: row.get_column_by_name("address")
                .and_then(|v| v.get_string())
                .map_err(|e| e.to_string())?,
            created_at: row.get_column_by_name("created_at")
                .and_then(|v| v.get_i64())
                .map_err(|e| e.to_string())?,
            updated_at: row.get_column_by_name("updated_at")
                .and_then(|v| v.get_i64())
                .map_err(|e| e.to_string())?,
        };
        
        events.push(event);
    }
    Ok(events)
}

#[tauri::command]
pub async fn list_events_by_creator_id(creator_id: Uuid) -> Result<Vec<Event>> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;
    let db_name = "openmeet.events";

    let query = format!("SELECT * FROM {} WHERE creator_id = ?;", db_name);
    let mut statement = session.statement(query.clone());
    statement.bind_by_name("creator_id", creator_id)?;
    let result = statement.execute().await.map_err(|e| e.to_string())?;

    let mut events = Vec::new();

    let mut iterator = result.iter();

    while let Some(row) = iterator.next() {
        let event = Event {
            event_id: row.get_column_by_name("event_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            creator_id: row.get_column_by_name("creator_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            title: row.get_column_by_name("title").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            description: row.get_column_by_name("description").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            start_time: row.get_column_by_name("start_time").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            end_time: row.get_column_by_name("end_time").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            lat: row.get_column_by_name("lat").and_then(|v| v.get_f64()).map_err(|e| e.to_string())?,
            lon: row.get_column_by_name("lon").and_then(|v| v.get_f64()).map_err(|e| e.to_string())?,
            address: row.get_column_by_name("address").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            created_at: row.get_column_by_name("created_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
            updated_at: row.get_column_by_name("updated_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?,
        };
        events.push(event);
    }

    Ok(events)
}


#[tauri::command]
pub async fn delete_events_by_creator_id(creator_id: Uuid) -> Result<()> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;
    let db_name = "openmeet.events";

    let query = format!("DELETE FROM {} WHERE creator_id = ?;", db_name);
    let mut statement = session.statement(query.clone());
    statement.bind_by_name("creator_id", creator_id)?;

    statement.execute().await.map_err(|e| e.to_string())?;

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    // disable test
    #[tokio::test]
    async fn test_list_events() {
        // get currrent, possbly empty list
        let events = list_events().await.unwrap();
        let events_count = events.len();

        let uuid_gen = UuidGen::default();
        let creator_id = uuid_gen.gen_time();
        let timestamp_gen = TimestampGen::gen_monotonic_new();

        // insert an event
        let event = create_event(
            creator_id, 
            "title".to_string(),
            "description".to_string(),
            5000,
            10000,
           0.0,
           0.0,
           "address".to_string()).await.unwrap();

        // at least one more record now
        let events = list_events().await.unwrap();
        assert!(events.len() >= events_count + 1);
    }

    #[tokio::test]
    async fn test_connect() {
        let mut cluster = init_cluster().await.unwrap();
        let result = cluster.connect().await;
        // session has no error
        assert!(result.is_ok());
        assert!(result.err().is_none());
    }
     
    #[tokio::test]
    async fn test_create_event() {
        let uuid_gen = UuidGen::default();
        let creator_id = uuid_gen.gen_time();

        let event = create_event(
            creator_id, 
            "title is good".to_string(),
            "description".to_string(),
            15000,
            150000,
            0.0,
            0.0,
            "address".to_string()
        ).await.unwrap();   

        println!("looking up creator_id: {:?}, start_time: {:?}, event_id: {:?}", creator_id, event.start_time, event.event_id);
        // look it back up
        let event = get_event(creator_id, event.start_time, event.event_id).await.unwrap();

        assert_eq!(event.unwrap().title, "title is good");
    }

    #[tokio::test]
    async fn test_delete_events_by_creator_id() {
        // select a random creator_id
        let events = list_events().await.unwrap();
        if events.is_empty() {
            return;
        }
        let creator_id = events[0].creator_id;
        
        // check inital record count
        let events = list_events_by_creator_id(creator_id).await.unwrap();
        let events_count = events.len();
        assert!(events_count > 0);

        // delete all events with that creator_id
        delete_events_by_creator_id(creator_id).await.unwrap();

        // assert that the count of events is 0 for that creator_id
        let events_for_creator = list_events_by_creator_id(creator_id).await.unwrap();
        assert_eq!(events_for_creator.len(), 0);


    }
}
