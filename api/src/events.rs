use crate::middleware::auth::AuthToken;
use cassandra_cpp::{BindRustType, LendingIterator, Session, Statement};
use chrono::{DateTime, Utc};
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{delete, get, post, put};
use uuid::{uuid, Uuid};
use crate::cassandra_pool::{CassandraConnection, CassandraPool};
use crate::get_connection;

#[derive(Debug, Serialize, Deserialize)]
pub struct EventDeleteRequest {
    pub event_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Event {
    event_id: Uuid,
    group_id: Uuid,
    title: String,
    description: String,
    start_time: i64,
    end_time: i64,
    lat: f64,
    lon: f64,
    location: String,
    created_at: i64,
    updated_at: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateEventRequest {
    title: String,
    description: String,
    start_time: String,
    end_time: String,
    lat: f64,
    lon: f64,
    location: String,
    group_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeleteEventRequest {
    user_id: Uuid,
    event: Event,
}

#[post("/events", data = "<event>")]
pub async fn frontend_create_event(conn: CassandraConnection, event: Json<CreateEventRequest>) -> Result<Json<Event>, Status> {
    println!("Creating event: {:?}", event);

    let start_time = DateTime::parse_from_rfc3339(&event.start_time).unwrap();
    let end_time = DateTime::parse_from_rfc3339(&event.end_time).unwrap();

    let new_event = Event {
        event_id: Uuid::new_v4(),
        group_id: event.group_id.clone(),
        title: event.title.clone(),
        description: event.description.clone(),
        start_time: start_time.timestamp_millis(),
        end_time: end_time.timestamp_millis(),
        lat: event.lat,
        lon: event.lon,
        location: event.location.clone(),
        created_at: Utc::now().timestamp_millis(),
        updated_at: Utc::now().timestamp_millis(),
    };

    match create_event(conn, &new_event).await {
        Ok(_) => Ok(Json(new_event)),
        Err(_) => Err(Status::InternalServerError),
    }
}

async fn create_event(conn: CassandraConnection, event: &Event) -> Result<Json<Event>, Status> {
    println!("Creating new event wanted event_id: {:?}", event.event_id);
    println!(
        "Creating new event wanted group_id: {:?}",
        event.group_id
    );
    println!(
        "Creating new event wanted start_time: {:?}",
        event.start_time
    );
    let new_event_id = Uuid::new_v4();
    println!("but got: {:?}", new_event_id);
    let session = conn;
    
    let db_name = "openmeet.events";
    let insert_event_query = format!("INSERT INTO {} (event_id, group_id, title, description, start_time, end_time, lat, lon, location, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?);", db_name);

    let mut statement = session.statement(&insert_event_query);
    statement
        .bind(0, new_event_id)
        .map_err(|_| Status::InternalServerError)?;
    // we should look this up from token or something else
    statement
        .bind(1, event.group_id)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(2, event.title.as_str())
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(3, event.description.as_str())
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(4, event.start_time)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(5, event.end_time)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(6, event.lat)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(7, event.lon)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(8, event.location.as_str())
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(9, event.created_at)
        .map_err(|_| Status::InternalServerError)?;
    statement
        .bind(10, event.updated_at)
        .map_err(|_| Status::InternalServerError)?;

    if let Err(e) = statement.execute().await {
        println!("Failed to execute statement: {:?}", e);
        return Err(Status::InternalServerError);
    }

    let mut new_event = event.clone();
    new_event.event_id = new_event_id;
    Ok(Json(new_event))
}

// #[get("/events")]
// pub async fn get_events(db: &State<DbConn>) -> Result<Json<Vec<Event>>, Status> {
//     match db.get_events().await {
//         Ok(events) => Ok(Json(events)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

#[get("/events/<event_id>")]
pub async fn frontend_get_event(conn: CassandraConnection, event_id: &str) -> Result<Json<Event>, Status> {
    let event_id = Uuid::parse_str(event_id).map_err(|_| Status::BadRequest)?;
    match get_event(conn, event_id, Uuid::new_v4(), Utc::now().timestamp_millis()).await {
        Ok(Some(event)) => Ok(Json(event)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError),
    }
}

async fn get_events_by_group_id(conn: CassandraConnection, group_id: Uuid) -> Result<Json<Vec<Event>>, Status> {
    let mut events = Vec::new();
    let session = conn;

    let db_name = "openmeet.events";
    let select_events_query = format!("SELECT event_id, group_id, title, description, start_time, end_time, lat, lon, location, created_at, updated_at FROM {} WHERE group_id = ?;", db_name);

    let mut statement = session.statement(&select_events_query);
    statement
        .bind(0, group_id)
        .map_err(|_| Status::InternalServerError)?;

    let result = statement
        .execute()
        .await
        .map_err(|_| Status::InternalServerError)?;

    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        let event_id = match row.get_column_by_name("event_id") {
            Ok(col) => match col.get_uuid() {
                Ok(uuid) => {
                    println!("event_id: {:?}", uuid);

                    uuid
                }
                Err(e) => {
                    eprintln!("Failed to parse event_id as UUID: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            },
            Err(e) => {
                eprintln!("Failed to get event_id as string: {:?}", e);
                return Err(Status::InternalServerError);
            }
        };

        let group_id = match row.get_column_by_name("group_id") {
            Ok(col) => match col.get_uuid() {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Failed to parse group_id as UUID: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            },
            Err(e) => {
                eprintln!("Failed to get group_id: {:?}", e);
                return Err(Status::InternalServerError);
            }
        };

        events.push(Event {
            event_id: event_id.into(),
            group_id: group_id.into(),
            title: row
                .get_column_by_name("title")
                .unwrap()
                .get_string()
                .unwrap(),
            description: row
                .get_column_by_name("description")
                .unwrap()
                .get_string()
                .unwrap(),
            start_time: row
                .get_column_by_name("start_time")
                .unwrap()
                .get_i64()
                .unwrap(),
            end_time: row
                .get_column_by_name("end_time")
                .unwrap()
                .get_i64()
                .unwrap(),
            lat: row.get_column_by_name("lat").unwrap().get_f64().unwrap(),
            lon: row.get_column_by_name("lon").unwrap().get_f64().unwrap(),
            location: row
                .get_column_by_name("location")
                .unwrap()
                .get_string()
                .unwrap(),
            created_at: row
                .get_column_by_name("created_at")
                .unwrap()
                .get_i64()
                .unwrap(),
            updated_at: row
                .get_column_by_name("updated_at")
                .unwrap()
                .get_i64()
                .unwrap(),
        });
    }

    Ok(Json(events))
}

async fn get_event(
    conn: CassandraConnection,
    event_id: Uuid,
    group_id: Uuid,
    start_time: i64,
) -> Result<Option<Event>, Status> {
    let session = conn;

    let db_name = "openmeet.events";
    let select_event_query = format!("SELECT event_id, group_id, title, description, start_time, end_time, lat, lon, location, created_at, updated_at FROM {} WHERE event_id = ? AND group_id = ? AND start_time = ?;", db_name);

    let mut statement = session.statement(&select_event_query);
    statement.bind(0, event_id).map_err(|e| {
        eprintln!("Failed to bind event_id: {:?}", e);
        Status::InternalServerError
    })?;
    statement.bind(1, group_id).map_err(|e| {
        eprintln!("Failed to bind group_id: {:?}", e);
        Status::InternalServerError
    })?;
    statement.bind(2, start_time).map_err(|e| {
        eprintln!("Failed to bind start_time: {:?}", e);
        Status::InternalServerError
    })?;
    let result = statement.execute().await.map_err(|e| {
        eprintln!("Failed to execute select statement: {:?}", e);
        Status::InternalServerError
    })?;

    let mut events = Vec::new();
    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        let event_id = match row.get_column_by_name("event_id") {
            Ok(col) => match col.get_uuid() {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Failed to parse event_id as UUID: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            },
            Err(e) => {
                eprintln!("Failed to get event_id: {:?}", e);
                return Err(Status::InternalServerError);
            }
        };

        let group_id = match row.get_column_by_name("group_id") {
            Ok(col) => match col.get_uuid() {
                Ok(uuid) => uuid,
                Err(e) => {
                    eprintln!("Failed to parse group_id as UUID: {:?}", e);
                    return Err(Status::InternalServerError);
                }
            },
            Err(e) => {
                eprintln!("Failed to get group_id: {:?}", e);
                return Err(Status::InternalServerError);
            }
        };

        let event = Event {
            event_id: event_id.into(),
            group_id: group_id.into(),
            title: row
                .get_column_by_name("title")
                .unwrap()
                .get_string()
                .unwrap(),
            description: row
                .get_column_by_name("description")
                .unwrap()
                .get_string()
                .unwrap(),
            start_time: row
                .get_column_by_name("start_time")
                .unwrap()
                .get_i64()
                .unwrap(),
            end_time: row
                .get_column_by_name("end_time")
                .unwrap()
                .get_i64()
                .unwrap(),
            lat: row.get_column_by_name("lat").unwrap().get_f64().unwrap(),
            lon: row.get_column_by_name("lon").unwrap().get_f64().unwrap(),
            location: row
                .get_column_by_name("location")
                .unwrap()
                .get_string()
                .unwrap(),
            created_at: row
                .get_column_by_name("created_at")
                .unwrap()
                .get_i64()
                .unwrap(),
            updated_at: row
                .get_column_by_name("updated_at")
                .unwrap()
                .get_i64()
                .unwrap(),
        };
        events.push(event); // Push each event into the vector
    }
    Ok(events.get(0).cloned())
}

#[delete("/events/<event_id>", data = "<delete_event_request>")]
pub async fn frontend_delete_event(
    conn: CassandraConnection,
    _auth: AuthToken,
    event_id: &str,
    delete_event_request: Json<DeleteEventRequest>,
) -> Status {
    println!("Deleting event: {:?}", event_id);

    let event_id = Uuid::parse_str(event_id).map_err(|_| Status::BadRequest);
    let user_id = delete_event_request.user_id;
    let event = delete_event_request.event.clone();

    if event.group_id != user_id {
        return Status::Forbidden;
    }
    let event_id = event_id.unwrap();
    match delete_event(conn, &event_id, &user_id, &event.start_time).await {
        Ok(_) => Status::NoContent,
        Err(_) => Status::InternalServerError,
    }
}

pub async fn delete_event(conn: CassandraConnection, event_id: &Uuid, user_id: &Uuid, start_date: &i64) -> Result<(), Status> {
    println!("Deleting event: {:?}", event_id);

    let session = conn;

    let delete_query =
        "DELETE FROM openmeet.events WHERE event_id = ? and start_time = ? and group_id = ?";
    let mut statement = session.statement(delete_query);
    statement.bind(0, *event_id).map_err(|e| {
        eprintln!("Failed to bind event_id: {:?}", e);
        Status::InternalServerError
    })?;
    statement.bind(1, *start_date).map_err(|e| {
        eprintln!("Failed to bind start_time: {:?}", e);
        Status::InternalServerError
    })?;
    statement.bind(2, *user_id).map_err(|e| {
        eprintln!("Failed to bind group_id: {:?}", e);
        Status::InternalServerError
    })?;

    let result = match statement.execute().await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Failed to execute delete statement: {:?}", e);
            return Err(Status::InternalServerError);
        }
    };

    if result.row_count() == 0 {
        eprintln!(
            "No event found with event_id: {:?}, start_time: {:?}, and group_id: {:?}",
            event_id, start_date, user_id
        );
        return Err(Status::NotFound);
    }

    Ok(())
}
// #[put("/events/<event_id>", data = "<event>")]
// pub async fn update_event(event_id: Uuid, event: Json<CreateEventRequest>, db: &State<DbConn>, user_id: Uuid) -> Result<Json<Event>, Status> {
//     let updated_event = Event {
//         event_id,
//         group_id: user_id,
//         title: event.title.clone(),
//         description: event.description.clone(),
//         start_time: event.start_time,
//         end_time: event.end_time,
//         lat: event.lat,
//         lon: event.lon,
//         location: event.location.clone(),
//         created_at: Utc::now(), // You might want to keep the original created_at
//         updated_at: Utc::now(),
//     };

//     match db.update_event(&updated_event).await {
//         Ok(_) => Ok(Json(updated_event)),
//         Err(_) => Err(Status::InternalServerError),
//     }
// }

// #[delete("/events/<event_id>")]
// pub async fn delete_event(event_id: Uuid, db: &State<DbConn>, user_id: Uuid) -> Status {
//     match db.delete_event(event_id, user_id).await {
//         Ok(_) => Status::NoContent,
//         Err(_) => Status::InternalServerError,
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    #[tokio::test]
    async fn test_get_events_by_group_id_success() {
        let pool = CassandraPool::new("127.0.0.1:9042").await.unwrap();
        let cassandra_conn = get_connection(&pool).await.unwrap();
        
                // let group_id = Uuid::parse_str("115c9dbd-ccfb-43cd-8341-0f242144c98f").unwrap();

        // // create 3 events by same creator
        // let mut preload_events: Vec<Event> = Vec::new();
        // for i in 0..3 {
        //     let event = Event {
        //         event_id: Uuid::new_v4(),
        //         group_id,
        //         title: format!("Test Event {}", i + 1),
        //         description: format!("This is test event number {}", i + 1),
        //         start_time: Utc::now().timestamp_millis() + i * 3600000, // each event starts 1 hour after the previous one
        //         end_time: Utc::now().timestamp_millis() + (i + 1) * 3600000, // each event ends 1 hour after it starts
        //         lat: 40.7128 + i as f64 * 0.01, // slightly different latitudes
        //         lon: -74.0060 + i as f64 * 0.01, // slightly different longitudes
        //         location: format!("New York, NY {}", i + 1),
        //         created_at: Utc::now().timestamp_millis(),
        //         updated_at: Utc::now().timestamp_millis(),
        //     };
        //     match create_event(conn, &event).await {
        //         Ok(_) => (),
        //         Err(e) => {
        //             panic!("Failed to create event: {:?}", e);
        //         }
        //     }
            
        // }
        return ()
        // let events = match get_events_by_group_id(conn, group_id).await {
        //     Ok(events) => {
        //         println!("events len({:?}): {:?}", events.len(), events);
        //         let events = events.into_inner();
        //         assert!(events.len() >= 3);
        //         events
        //     }
        //     Err(e) => {
        //         panic!("Failed to get events: {:?}", e);
        //     }
        // };
    }

    #[tokio::test]
    async fn test_delete_event_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();

        let group_id = Uuid::parse_str("115c9dbd-ccfb-43cd-8341-0f242144c98f").unwrap();
        let start_time: i64 = 1725385197884;

        // look up the event from the test_create_event_success test and delete if it exists
        let events = match get_events_by_group_id(conn, group_id).await {
            Ok(events) => {
                let events = events.into_inner();
                let event = events.get(0).unwrap();
                let delete_result = delete_event(conn, &event.event_id, &group_id, &start_time).await;


            }
            Err(e) => {
                panic!("Failed to get events: {:?}", e);
            }
        };
    }

    #[tokio::test]
    async fn test_create_event_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();

        // Setup: create an event
        let event = Event {
            event_id: Uuid::parse_str("38408cb9-9c13-4ca7-ad78-c322bb2a38a9").unwrap(),
            group_id: Uuid::parse_str("70be065f-3251-4794-aa25-22b257abe977").unwrap(),
            title: "Test Event".to_string(),
            description: "This is a test event".to_string(),
            start_time: 1725385197884,
            end_time: 1725385197884 + 3600000, // 1 hour later
            lat: 40.7128,
            lon: -74.0060,
            location: "New York, NY".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };

        // Act: create the event
        let create_result = create_event(conn, &event).await;
        if create_result.is_err() {
            panic!("Failed to create test event: {:?}", create_result.err());
        }
        assert!(create_result.is_ok());
    }

    #[tokio::test]
    async fn test_get_event_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();

        // Setup: create an event and insert it into the database
        let event = Event {
            event_id: Uuid::new_v4(),
            group_id: Uuid::new_v4(),
            title: "Test Event".to_string(),
            description: "This is a test event".to_string(),
            start_time: Utc::now().timestamp_millis(),
            end_time: Utc::now().timestamp_millis() + 3600000, // 1 hour later
            lat: 40.7128,
            lon: -74.0060,
            location: "New York, NY".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
        };

        // Create the event
        let create_result = create_event(conn, &event).await;

        let create_result = match create_result {
            Ok(event) => event,
            Err(e) => {
                panic!("Failed to create test event: {:?}", e);
            }
        };

        // Act: get the event by id
        let result = get_event(
            conn,
            create_result.into_inner().event_id,
            event.group_id,
            event.start_time,
        )
        .await;

        match result {
            Ok(Some(retrieved_event)) => {
                assert_ne!(retrieved_event.event_id, event.event_id);
                assert_eq!(retrieved_event.title, event.title);
                assert_eq!(retrieved_event.description, event.description);
            }
            Ok(None) => {
                println!("Failed to retrieve test event");
            }
            Err(e) => {
                panic!("Failed to get test event: {:?}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_event_not_found() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();

        // Act: attempt to get a non-existent event
        let result = get_event(
            conn,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Utc::now().timestamp_millis(),
        )
        .await;

        // Assert: check that the result is Ok(None)
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
