use crate::init_cluster;
use bcrypt::{hash, verify, DEFAULT_COST};
use cassandra_cpp::BindRustType;
use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();
    email_regex.is_match(email)
}

// passed a user with unencrypted password, that becomes a bcrypted password_hash
pub async fn create_user(original_user: User) -> Result<User, String> {
    let cloned_original_user: User = original_user.clone();
    let mut user = original_user;

    // create bcrypted password_hash
    user.password_hash = hash(&user.password_hash, DEFAULT_COST).map_err(|e| e.to_string())?;

    // create uuid
    user.user_id = Uuid::new_v4();

    // check email is valid
    if !is_valid_email(&user.email).await {
        return Err("Invalid email".to_string());
    }

    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    // Check if email exists
    let email_check_query = "SELECT user_id FROM openmeet.email_index WHERE email = ?";
    let mut email_check_statement = session.statement(email_check_query);
    email_check_statement.bind(0, user.email.as_str()).map_err(|e| e.to_string())?;
    let email_check_result = email_check_statement.execute().await.map_err(|e| e.to_string())?;
    if email_check_result.first_row().is_some() {
        return Err("Email already exists".to_string());
    }

    let query = "INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let mut statement = session.statement(query);

    statement.bind(0, user.user_id).map_err(|e| e.to_string())?;
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
    assert_ne!(user.user_id, cloned_original_user.user_id);
    assert_ne!(user.password_hash, cloned_original_user.password_hash);

    let result = statement.execute().await;


    if let Err(e) = result {
        // Check if the error is due to a duplicate email
        println!("error: {:?}", e.to_string());
        if e.to_string().contains("duplicate") {
            return Err("Email already exists".to_string());
        }
        return Err(format!("Failed to create user {}", e.to_string()));
    }

    
    if result.is_ok() {
        let insert_email_index_query = "INSERT INTO openmeet.email_index (email, user_id) VALUES (?, ?)";
        let mut insert_email_index_statement = session.statement(insert_email_index_query);
        insert_email_index_statement.bind(0, user.email.as_str()).map_err(|e| e.to_string())?;
        insert_email_index_statement.bind(1, user.user_id).map_err(|e| e.to_string())?;
        let result = insert_email_index_statement.execute().await;
        if result.is_err() {
            return Err(format!("Failed to create user {}", result.err().unwrap()));
        }
    }

    println!("result: {:?}", result);
    user.password_hash = "xxxxxxxx".to_string();
    Ok(user)
}

pub async fn get_user_by_email(email: &str) -> Option<User> {
    let mut cluster = init_cluster().await.ok()?;
    let session = cluster.connect().await.ok()?;

    let query = "SELECT * FROM openmeet.users WHERE email = ?";
    let mut statement = session.statement(query);
    statement.bind(0, email).ok()?;

    let result = statement.execute().await.ok()?;

    if let Some(row) = result.first_row() {
        let user = User {
            user_id: row
                .get_column_by_name("user_id")
                .and_then(|v| v.get_uuid().map(|uuid| uuid.into()))
                .ok()?,
            username: row
                .get_column_by_name("username")
                .and_then(|v| v.get_string())
                .ok()?,
            email: row
                .get_column_by_name("email")
                .and_then(|v| v.get_string())
                .ok()?,
            password_hash: row
                .get_column_by_name("password_hash")
                .and_then(|v| v.get_string())
                .ok()?,
            created_at: DateTime::from_timestamp_millis(
                row.get_column_by_name("created_at")
                    .and_then(|v| v.get_i64())
                    .ok()?,
            )
            .unwrap(),
            updated_at: DateTime::from_timestamp_millis(
                row.get_column_by_name("updated_at")
                    .and_then(|v| v.get_i64())
                    .ok()?,
            )
            .unwrap(),
            last_login: DateTime::from_timestamp_millis(
                row.get_column_by_name("last_login")
                    .and_then(|v| v.get_i64())
                    .ok()?,
            )
            .unwrap(),
        };
        Some(user)
    } else {
        None
    }
}

pub async fn login(email: &str, password: &str) -> Result<User, String> {
    // create bcrypted password_hash
    let password_hash = hash(password, DEFAULT_COST).map_err(|e| e.to_string())?;
    if let Some(user) = get_user_by_email(email).await {
        if verify(&password_hash, &user.password_hash).map_err(|e| e.to_string())? {
            Ok(user)
        } else {
            Err("Invalid password".to_string())
        }
    } else {
        Err("User not found".to_string())
    }
}

pub async fn delete_user(user_id: &str) -> Result<(), String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "DELETE FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, user_id).map_err(|e| e.to_string())?;

    statement.execute().await.map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_login_success() {
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        create_user(user.clone())
            .await
            .expect("Failed to create user");

        // Act: attempt to login with correct credentials
        let result = login("testuser@example.com", "password123").await;

        // Assert: check that login was successful
        assert!(result.is_ok());
        let logged_in_user = result.unwrap();
        assert_eq!(logged_in_user.email, user.email);
    }

    #[tokio::test]
    async fn test_login_failure_wrong_password() {
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        create_user(user.clone())
            .await
            .expect("Failed to create user");

        // Act: attempt to login with incorrect password
        let result = login("testuser@example.com", "wrongpassword").await;

        // Assert: check that login failed
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_failure_nonexistent_user() {
        // Act: attempt to login with a non-existent user
        let result = login("nonexistent@example.com", "password123").await;

        // Assert: check that login failed
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_success() {
        // Setup: create a user instance
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserSUCCESS@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        // Act: create the user
        let result = create_user(user.clone()).await;

        match result {
            Ok(created_user) => {
                // Assert: check that user creation was successful
                assert_eq!(created_user.username, user.username);
                assert_eq!(created_user.email, user.email);
                assert_ne!(created_user.user_id, user.user_id);
                assert_ne!(created_user.password_hash, user.password_hash);
            }
            Err(e) => {
                if !e.to_string().contains("Email already exists") {
                    panic!("Expected error containing 'Email already exists', but got: {}", e.to_string());
                }
            }
        }
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        // Setup: create a user instance

        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserDUPLICATE@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        // Act: create the user
        let result = create_user(user.clone()).await;
        
        if let Err(e) = result {
            if !e.to_string().contains("Email already exists") {
                panic!("Expected error containing 'Email already exists', but got: {}", e.to_string());
            }
        }
        // Attempt to create another user with the same email
        let duplicate_user = User {
            user_id: Uuid::new_v4(),
            username: "anotheruser".to_string(),
            email: "testuserDUPLICATE@example.com".to_string(),
            password_hash: "password456".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        let result = create_user(duplicate_user).await;

        // Assert: check that user creation failed due to duplicate email
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        // Setup: create a user instance with an invalid email
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        // Act: attempt to create the user
        let result = create_user(user).await;

        // Assert: check that user creation failed due to invalid email
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        // Setup: create a user and insert into the database
        let user_sample = User {
            user_id: Uuid::new_v4(),
            username: "testuser1".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_login: Utc::now(),
        };

        create_user(user_sample.clone())
            .await
            .expect("Failed to create user");
        let user = get_user_by_email(&user_sample.email).await;
        assert!(user.is_some());

        // Act: delete the user
        let result = delete_user(&user.unwrap().user_id.to_string()).await;

        println!("result: {:?}", result);
        // Assert: check that user deletion was successful
        assert!(result.is_ok());

        // Verify that the user no longer exists
        let user = get_user_by_email(&user_sample.email).await;
        assert!(user.is_none());
    }
}
