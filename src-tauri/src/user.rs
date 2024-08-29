use std::fmt::Debug;

use crate::commands::init_cluster;
use cassandra_cpp::BindRustType;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// CREATE TABLE users (
//     user_id UUID PRIMARY KEY,
//     username TEXT,
//     email TEXT,
//     password_hash TEXT,
//     created_at TIMESTAMP,
//     updated_at TIMESTAMP,
//     last_login TIMESTAMP
//   );

#[derive(Debug)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}

#[tauri::command]
pub async fn create_user(user: User) -> Result<User, String> {
    println!("in create {:?}", user.user_id);
    let mut cluster = init_cluster().await;
    let session = cluster.connect().await.map_err(|e| e.to_string())?; // Await the connect method

    let query = "INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let prepared = session.prepare(query).await.map_err(|e| e.to_string())?;

    let mut statement = prepared.bind();
    statement
        .bind(0, user.user_id)
        .map_err(|e| e.to_string())?;
    statement
        .bind(1, user.username.as_str())
        .map_err(|e| e.to_string())?;
    statement
        .bind(2, user.email.as_str())
        .map_err(|e| e.to_string())?;
    statement
        .bind(3, user.password_hash.as_str())
        .map_err(|e| e.to_string())?;
    statement
        .bind(4, user.created_at.timestamp_millis())
        .map_err(|e| e.to_string())?;
    statement
        .bind(5, user.updated_at.timestamp_millis())
        .map_err(|e| e.to_string())?;
    statement
        .bind(6, user.last_login.timestamp_millis())
        .map_err(|e| e.to_string())?;

    println!("statement {:?}", statement);
    let result = session.execute(query).await.map_err(|e| e.to_string())?;
    println!("row count {:?}", result.row_count());
    println!("result {:?}", result.to_string());
    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_stuff() {
        assert_ne!(Uuid::new_v4(), Uuid::nil());
    }

    #[tokio::test]
    async fn test_create_user() {
        let user = User {
            user_id: Uuid::new_v4(),
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "test".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };
        println!("{:?}", user.user_id);
        let result = create_user(user).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
