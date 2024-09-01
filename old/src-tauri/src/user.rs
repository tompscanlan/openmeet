use std::fmt::Debug;

use crate::init_cluster;
use cassandra_cpp::BindRustType;
use chrono::{DateTime, Utc};
use cassandra_cpp::Uuid;
use cassandra_cpp::UuidGen;
use cassandra_cpp::LendingIterator;
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
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let mut statement = session.statement(query);

    statement.bind(0, user.user_id).map_err(|e| e.to_string())?;
    statement.bind(1, user.username.as_str()).map_err(|e| e.to_string())?;
    statement.bind(2, user.email.as_str()).map_err(|e| e.to_string())?;
    statement.bind(3, user.password_hash.as_str()).map_err(|e| e.to_string())?;
    statement.bind(4, user.created_at.timestamp_millis()).map_err(|e| e.to_string())?;
    statement.bind(5, user.updated_at.timestamp_millis()).map_err(|e| e.to_string())?;
    statement.bind(6, user.last_login.timestamp_millis()).map_err(|e| e.to_string())?;

    statement.execute().await.map_err(|e| e.to_string())?;

    Ok(user)
}
#[tauri::command]
pub async fn get_user(user_id: Uuid) -> Result<Option<User>, String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "SELECT * FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, user_id).map_err(|e| e.to_string())?;

    let result = statement.execute().await.map_err(|e| e.to_string())?;

    if let Some(row) = result.first_row() {
        let user = User {
            user_id: row.get_column_by_name("user_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            username: row.get_column_by_name("username").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            email: row.get_column_by_name("email").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            password_hash: row.get_column_by_name("password_hash").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            created_at: DateTime::from_timestamp_millis(row.get_column_by_name("created_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
            updated_at: DateTime::from_timestamp_millis(row.get_column_by_name("updated_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
            last_login: DateTime::from_timestamp_millis(row.get_column_by_name("last_login").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
        };
        Ok(Some(user))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn delete_user(user_id: Uuid) -> Result<(), String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "DELETE FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, user_id).map_err(|e| e.to_string())?;

    statement.execute().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn get_users() -> Result<Vec<User>, String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "SELECT * FROM openmeet.users";
    let mut statement = session.statement(query);

    let result = statement.execute().await.map_err(|e| e.to_string())?;

    let mut users = Vec::new();

    let mut itr = result.iter();
    while let Some(row) = itr.next() {
        let user = User {
            user_id: row.get_column_by_name("user_id").and_then(|v| v.get_uuid()).map_err(|e| e.to_string())?,
            username: row.get_column_by_name("username").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            email: row.get_column_by_name("email").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            password_hash: row.get_column_by_name("password_hash").and_then(|v| v.get_string()).map_err(|e| e.to_string())?,
            created_at: DateTime::from_timestamp_millis(row.get_column_by_name("created_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
            updated_at: DateTime::from_timestamp_millis(row.get_column_by_name("updated_at").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
            last_login: DateTime::from_timestamp_millis(row.get_column_by_name("last_login").and_then(|v| v.get_i64()).map_err(|e| e.to_string())?).unwrap(),
        };
        users.push(user);
    }

    Ok(users)
}

#[tauri::command]
pub async fn delete_user_by_user_id(user_id: Uuid) -> Result<usize, String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "DELETE FROM openmeet.users where user_id = ?";
    let mut statement = session.statement(query);

    statement.bind(0, user_id).map_err(|e| e.to_string())?;
    let result = statement.execute().await.map_err(|e| e.to_string())?;

    if result.row_count() == 0 {
        return Err("User not found".to_string());
    }

    // return how many rows were affected
    Ok(result.row_count() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    
    #[tokio::test]
    async fn test_create_user() {
        let UUID_GEN: UuidGen = UuidGen::default();
        
        let user = User {
            user_id: UUID_GEN.gen_time(),
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

    #[tokio::test]
    async fn test_get_user_not_found() {
        let uuid_gen = UuidGen::default();

        let user_id = uuid_gen.gen_time();

        let result = get_user(user_id).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_get_user_found() {
        let users = get_users().await.unwrap();
        if users.is_empty() {
            return;
        }
        
        let user_id = users[0].user_id;
        
        let user_result = get_user(user_id).await;
        assert!(user_result.is_ok());
        assert!(user_result.unwrap().is_some());
    }

    #[tokio::test]
    async fn test_delete_user() {
        let users = get_users().await.unwrap();
        if users.is_empty() {
            return;
        }

        let user_id = users[0].user_id;

        let user_result = get_user(user_id).await;
        assert!(user_result.is_ok());
        assert!(user_result.unwrap().is_some());

        let delete_result = delete_user_by_user_id(user_id).await;
        assert!(delete_result.is_ok());

        let user_result = get_user(user_id).await;
        assert!(user_result.is_ok());
        assert!(user_result.unwrap().is_none());
        assert_eq!(delete_result.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_delete_user_by_user_id_not_found() {
        let uuid_gen = UuidGen::default();

        let user_id = uuid_gen.gen_time();

        let delete_result = delete_user_by_user_id(user_id).await;
        assert!(delete_result.is_ok());
        assert_eq!(delete_result.unwrap(), 0);
        
    }
}
