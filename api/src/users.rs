use crate::Json;
use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use cassandra_cpp::BindRustType;
use cassandra_cpp::LendingIterator;
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use regex::Regex;
use rocket::http::ext::IntoCollection;
use rocket::http::hyper::server::conn;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::env;
use cassandra_cpp::Cluster;
use crate::cassandra_pool::{CassandraConnection, CassandraPool};
use crate::get_connection;


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: Uuid,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub username: String,
    pub email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interests: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_login: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRegister {
    pub username: String,
    pub email: String,
    pub password: String,
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

async fn check_email_exists(session: &cassandra_cpp::Session, email: &str) -> Result<bool, String> {
    let email_check_query = "SELECT user_id FROM openmeet.email_index WHERE email = ?";
    let mut email_check_statement = session.statement(email_check_query);
    email_check_statement
        .bind(0, email)
        .map_err(|e| e.to_string())?;
    let email_check_result = email_check_statement
        .execute()
        .await
        .map_err(|e| e.to_string())?;
    Ok(email_check_result.first_row().is_some())
}

async fn insert_user(
    session: &cassandra_cpp::Session,
    user: &User,
    now: i64,
) -> Result<(), String> {
    let query = "INSERT INTO openmeet.users (user_id, username, email, password_hash, created_at, updated_at, last_login) VALUES (?, ?, ?, ?, ?, ?, ?)";
    let mut statement = session.statement(query);

    statement.bind_by_name("user_id", user.user_id).map_err(|e| e.to_string())?;
    statement
        .bind_by_name("username", user.username.as_str())
        .map_err(|e| e.to_string())?;
    statement
        .bind_by_name("email", user.email.as_str())
        .map_err(|e| e.to_string())?;
    statement
        .bind_by_name("password_hash", user.password_hash.clone().unwrap().as_str())
        .map_err(|e| e.to_string())?;
    statement.bind_by_name("created_at", now).map_err(|e| e.to_string())?;
    statement.bind_by_name("updated_at", now).map_err(|e| e.to_string())?;
    statement.bind_by_name("last_login", now).map_err(|e| e.to_string())?;

    statement.execute().await.map_err(|e| e.to_string())?;
    Ok(())
}
async fn insert_email_index(
    session: &cassandra_cpp::Session,
    email: &str,
    user_id: Uuid,
) -> Result<(), String> {
    let insert_email_index_query =
        "INSERT INTO openmeet.email_index (email, user_id) VALUES (?, ?)";
    let mut insert_email_index_statement = session.statement(insert_email_index_query);
    insert_email_index_statement
        .bind(0, email)
        .map_err(|e| e.to_string())?;
    insert_email_index_statement
        .bind(1, user_id)
        .map_err(|e| e.to_string())?;
    insert_email_index_statement
        .execute()
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

// passed a user with unencrypted password, that becomes a bcrypted password_hash
pub async fn create_user(conn: CassandraConnection, original_user: User) -> Result<User, String> {
    // let cloned_original_user: User = original_user.clone();
    let mut user = original_user;

    
    // create bcrypted password_hash
    user.password_hash = match user.password_hash {
        Some(password) => {
            match hash(password, DEFAULT_COST).map_err(|e| e.to_string()) {
                Ok(hashed) => Some(hashed),
                Err(e) => {
                    println!("hash error: {:?}", e);
                    return Err(e);
                }
            }
        },
        None => return Err("Password is required".to_string()),
    };


    // create uuid
    user.user_id = Uuid::new_v4();

    // check email is valid
    if !is_valid_email(&user.email).await {
        return Err("Invalid email".to_string());
    }

    // time now
    let now = Utc::now().timestamp_millis();

   
    let session = conn;

    if check_email_exists(&session, &user.email).await? {
        return Err("Email already exists".to_string());
    }
    match insert_user(&session, &user, now).await {
        Ok(_) => (),
        Err(e) => { 
            println!("insert_user error: {:?}", e);
            return Err(e.to_string());
        }
    }
    match insert_email_index(&session, &user.email, user.user_id).await {
        Ok(_) => (),
        Err(e) => {
            println!("insert_email_index error: {:?}", e);
            return Err(e.to_string());
        }
    }
    Ok(user)
}

pub async fn init_cluster() -> Result<Cluster, String> {
    let mut cluster = Cluster::default();
    let contact_points = env::var("CASSANDRA_CONTACT_POINTS")
        .map_err(|_| "CASSANDRA_CONTACT_POINTS environment variable not set".to_string())?;
    cluster
        .set_contact_points(&contact_points)
        .map_err(|e| format!("Failed to set contact points: {}", e))?;

    let username = env::var("CASSANDRA_USERNAME").unwrap_or_default();
    let password = env::var("CASSANDRA_PASSWORD").unwrap_or_default();

    if !username.is_empty() && !password.is_empty() {
        cluster
            .set_credentials(&username, &password)
            .map_err(|e| format!("Failed to set credentials: {}", e))?;
    }

    Ok(cluster)
}

pub async fn get_user_by_id(conn: CassandraConnection, user_id: Uuid) -> Option<User> {

    let query = "SELECT * FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, user_id).unwrap();

    let result = statement.execute().await.unwrap();
    let row = result.first_row()?;

    let description = row
        .get_column_by_name("description")
        .unwrap()
        .get_string()
        .ok();
    let interests = row.get_column_by_name("interests").unwrap().get_set().ok();
    let mut interests_iter = interests.unwrap();
    let mut interests_vec: Vec<String> = Vec::new();

    while let Some(interest) = interests_iter.next() {
        interests_vec.push(interest.to_string());
    }

    Some(User {
        user_id: row
            .get_column_by_name("user_id")
            .unwrap()
            .get_uuid()
            .unwrap()
            .into(),
        username: row
            .get_column_by_name("username")
            .unwrap()
            .get_string()
            .unwrap(),
        email: row
            .get_column_by_name("email")
            .unwrap()
            .get_string()
            .unwrap(),
            password_hash: row
            .get_column_by_name("password_hash")
            .unwrap()
            .get_string()
            .ok(),
        description: description,
        interests: interests_vec,
        created_at: Some(
            row.get_column_by_name("created_at")
                .unwrap()
                .get_i64()
                .unwrap(),
        ),
        updated_at: Some(
            row.get_column_by_name("updated_at")
                .unwrap()
                .get_i64()
                .unwrap(),
        ),
        last_login: Some(
            row.get_column_by_name("last_login")
                .unwrap()
                .get_i64()
                .unwrap(),
        ),
    })
}

pub async fn get_user_by_email(conn: CassandraConnection, email: &str) -> Result<Option<User>, String> {
    let mut cluster = init_cluster().await.unwrap();
    let session = cluster.connect().await.unwrap();

    let query = "SELECT * FROM openmeet.users WHERE email = ?";
    let mut statement = session.statement(query);
    statement.bind(0, email).unwrap();

    let result = statement.execute().await.map_err(|e| {
        format!("get_user_by_email select error: {:?}", e)
    })?;

    let row = match result.first_row() {
        Some(row) => row,
        None => return Ok(None), // User not found
    };

    let description = row
        .get_column_by_name("description")
        .unwrap()
        .get_string()
        .ok();
    let interests = row.get_column_by_name("interests").unwrap().get_set().ok();
    let mut interests_vec: Vec<String> = Vec::new();
    if let Some(mut set) = interests {
        while let Some(interest) = set.next() {
            interests_vec.push(interest.to_string());
        }
    }

    let user_id = match row.get_column_by_name("user_id").ok() {
        Some(col) => col.get_uuid().ok(),
        None => return Ok(None),
    };
    let username = match row.get_column_by_name("username").ok() {
        Some(col) => col.get_string().ok(),
        None => return Ok(None),
    };
    let email = match row.get_column_by_name("email").ok() {
        Some(col) => col.get_string().ok(),
        None => return Ok(None),
    };
    let password_hash = match row.get_column_by_name("password_hash").ok() {
        Some(col) => col.get_string().ok(),
        None => return Ok(None),
    };

    let created_at = match row.get_column_by_name("created_at").ok() {
        Some(col) => col.get_i64().ok(),
        None => return Ok(None),
    };
    let updated_at = match row.get_column_by_name("updated_at").ok() {
        Some(col) => col.get_i64().ok(),
        None => return Ok(None),
    };
    let last_login = match row.get_column_by_name("last_login").ok() {
        Some(col) => col.get_i64().ok(),
        None => return Ok(None),
    };

    Ok(Some(User {
        user_id: user_id.unwrap().into(),
        username: username.unwrap(),
        email: email.unwrap(),
        password_hash: password_hash,
        description: description,
        interests: interests_vec,
        created_at: created_at,
        updated_at: updated_at,
        last_login: last_login,
    }))
}

fn generate_token(user: &User) -> Result<String, String> {
    let claims = Claims {
        sub: user.user_id.to_string(),
        exp: (Utc::now().timestamp() + 3600) as usize, // Token valid for 1 hour
    };

    let encoding_key = EncodingKey::from_secret("your_secret_key".as_ref());
    encode(&Header::default(), &claims, &encoding_key).map_err(|e| e.to_string())
}

pub async fn login(conn: CassandraConnection, email: &str, password: &str) -> Result<String, String> {
    if let Ok(Some(user)) = get_user_by_email(conn, email).await {
        let user_clone = user.clone();
        let password_verified = match user.password_hash {
            Some(password_hash) => verify(password, &password_hash).map_err(|e| e.to_string()),
            None => Err("Invalid password".to_string())
        };
        match password_verified {
            Ok(true) => {
                let token = generate_token(&user_clone)?;
                Ok(token)
            }
            Ok(false) | Err(_) => Err("Invalid credentials".to_string()),
        }
    } else {
        Err("User not found".to_string())
    }
}

pub async fn delete_user(conn: CassandraConnection, user_id: &Uuid, email: &str) -> Result<(), String> {
    let user = match get_user_by_email(&conn, email).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err("User not found".to_string()),
        Err(e) => return Err(e),
    };
    
    let query = "DELETE FROM openmeet.users WHERE user_id = ?";
    let mut statement = conn.statement(query);
    statement.bind(0, *user_id).map_err(|e| e.to_string())?;
    match statement.execute().await.map_err(|e| e.to_string()) {
        Ok(_) => (),
        Err(e) => {
            println!("delete from users error: {:?}", e);
            return Err(e.to_string());
        }
    }

    // also delete from email_index
    let query = "DELETE FROM openmeet.email_index WHERE email = ?";
    let mut statement = conn.statement(query);
    statement.bind(0, email).map_err(|e| e.to_string())?;
    match statement.execute().await.map_err(|e| e.to_string()) {
        Ok(_) => (),
        Err(e) => {
            println!("delete from email_index error: {:?}", e);
            return Err(e.to_string());
        }
    }

    Ok(())
}
pub async fn get_all_users() -> Result<Vec<User>, String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let query = "SELECT * FROM openmeet.users";
    let statement = session.statement(query);
    let result = statement.execute().await.map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    let mut iter = result.iter();
    while let Some(row) = iter.next() {
        let description = row
            .get_column_by_name("description")
            .ok()
            .and_then(|col| col.get_string().ok())
            .unwrap_or_default();

        let interests = row.get_column_by_name("interests").unwrap().get_set().ok();
        let mut interests_iter = interests.unwrap();
        let mut interests_vec: Vec<String> = Vec::new();

        while let Some(interest) = interests_iter.next() {
            interests_vec.push(interest.to_string());
        }
        users.push(User {
            user_id: row
                .get_column_by_name("user_id")
                .ok()
                .and_then(|col| col.get_uuid().ok())
                .ok_or("Invalid user_id".to_string())?
                .into(),
            username: row
                .get_column_by_name("username")
                .ok()
                .and_then(|col| col.get_string().ok())
                .ok_or("Invalid username".to_string())?, // Convert Option to Result
            email: row
                .get_column_by_name("email")
                .ok()
                .and_then(|col| col.get_string().ok())
                .ok_or("Invalid email".to_string())?,
            password_hash: row.get_column_by_name("password_hash").ok().and_then(|col| col.get_string().ok()),
            description: Some(description),
            interests: interests_vec,
            created_at: Some(
                row.get_column_by_name("created_at")
                    .ok()
                    .and_then(|col| col.get_i64().ok())
                    .unwrap_or_default(),
            ),
            updated_at: Some(
                row.get_column_by_name("updated_at")
                    .ok()
                    .and_then(|col| col.get_i64().ok())
                    .unwrap_or_default(),
            ),
            last_login: Some(
                row.get_column_by_name("last_login")
                    .ok()
                    .and_then(|col| col.get_i64().ok())
                    .unwrap_or_default(),
            ),
        });
    }
    Ok(users)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_login_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();

        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserLOGIN_SUCCESS@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: None,
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        let created_user = create_user(conn,user.clone()).await;

        if let Err(e) = created_user {
            if !e.to_string().contains("Email already exists") {
                panic!("Failed to create user: {}", e);
            }
        }
        // Act: attempt to login with correct credentials
        let result = login(conn, &user.email, &user.password_hash.unwrap()).await;

        if let Err(e) = result {
            println!("result--->: {:?}", e);
        } else {
            let token = result.unwrap().clone();
            assert_eq!(token.len() > 0, true);
        }
    }

    #[tokio::test]
    async fn test_login_credentials_create_user() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user instance
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserCREATE_LOGIN@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        //  delete any user with this email
        match delete_user(conn, &user.user_id, &user.email).await {
            Ok(_) => (),
            Err(e) => {
                panic!("pre-cleanup delete_user error: {:?}", e);
            }
        }

        // Act: create the user
        let create_result = match create_user(conn, user.clone()).await {
            Ok(created_user) => created_user,
            Err(e) => {
                println!("create_result: {:?}", e);
                panic!("Failed to create user: {:?}", e);
            }
        };

        assert_eq!(create_result.email, user.email);

        // Act: attempt to login with the same user
        let login_result = login(conn, &user.email, &user.password_hash.unwrap()).await;

        // Assert: check that login was successful
        assert!(login_result.is_ok());
        let token = login_result.unwrap();
        assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_login_second_layer() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // in a loop
        // register a user with a random email and password
        // login with the user
        // delete the user
        for i in 0..5 {
            // Generate a random email and password
            let email = format!("testuser{}@example.com", i);
            let password = format!("password{}", i);

            let user = get_user_by_email(conn, email.as_str()).await;
            if let Ok(Some(user)) = user {
                let _ = delete_user(conn,&user.user_id, &user.email).await;
            }

            // Setup: create a user instance
            let user = User {
                user_id: Uuid::new_v4(),
                username: format!("testuser{}", i),
                email: email.clone(),
                password_hash: Some(password.clone()),
                description: Some("Some long description".to_string()),
                interests: Vec::new(),
                created_at: None,
                updated_at: None,
                last_login: None,
            };

            // Act: create the user
            let create_result = crate::register(conn, Json(UserRegister {
                username: user.username.clone(),
                email: email.clone(),
                password: password.clone(),
            }))
            .await;
            assert!(create_result.is_ok());

            // Act: attempt to login with the same user
            let login_result = crate::frontend_login(conn, Json(UserLogin {
                email: email.clone(),
                password: password.clone(),
            }))
            .await;

            let token = login_result.into_inner();
            // Assert: check that login was successful
            assert!(!token.is_string(), "Token should be a string");
            // Act: delete the user
            let delete_result = delete_user(conn, &user.user_id, &user.email).await;
            assert!(delete_result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_login_failure_wrong_password() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: None,
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        // ignore any duplicate errors
        let _ = create_user(conn, user.clone()).await;

        // Act: attempt to login with incorrect password
        let result = login(conn, "testuser@example.com", "wrongpassword").await;

        // Assert: check that login failed
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_login_failure_nonexistent_user() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Act: attempt to login with a non-existent user
        let result = login(conn, "nonexistent@example.com", "password123").await;

        // Assert: check that login failed
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user instance
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserSUCCESS@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        let _ = delete_user(conn, &user.user_id, &user.email).await;

        // Act: create the user
        let result = create_user(conn, user.clone()).await;

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
                    panic!(
                        "Expected error containing 'Email already exists', but got: {}",
                        e.to_string()
                    );
                }
            }
        }
    }

    #[tokio::test]
    async fn test_create_user_duplicate_email() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user instance
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserDUPLICATE@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        // Act: create the user
        let result = create_user(conn, user.clone()).await;

        if let Err(e) = result {
            if !e.to_string().contains("Email already exists") {
                panic!(
                    "Expected error containing 'Email already exists', but got: {}",
                    e.to_string()
                );
            }
        }
        // Attempt to create another user with the same email
        let duplicate_user = User {
            user_id: Uuid::new_v4(),
            username: "anotheruser".to_string(),
            email: "testuserDUPLICATE@example.com".to_string(),
            password_hash: Some("password456".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        let result = create_user(conn, duplicate_user).await;

        // Assert: check that user creation failed due to duplicate email
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_create_user_invalid_email() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user instance with an invalid email
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "invalid-email".to_string(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        // Act: attempt to create the user
        let result = create_user(conn, user).await;

        // Assert: check that user creation failed due to invalid email
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user and insert into the database
        let random_email = format!("{uuid}@example.com", uuid = Uuid::new_v4());
        let user_sample = User {
            user_id: Uuid::new_v4(),
            username: random_email.clone(),
            email: random_email.clone(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        let create_result = create_user(conn, user_sample.clone()).await;
        assert!(create_result.is_ok());

        let user_id = create_result.unwrap().user_id;

        // Act: delete the user
        let delete_result = delete_user(conn, &user_id, &user_sample.email).await;
        if let Err(e) = delete_result.clone() {
            println!("delete_result: {:?}", e);
        }
        // Assert: check that user deletion was successful
        assert!(delete_result.is_ok(), "User deletion should succeed");

        // Verify that the user no longer exists
        let user_after_deletion = get_user_by_email(conn, &user_sample.email).await;
        if let Ok(None) = user_after_deletion {
            assert!(user_after_deletion.is_ok());
            assert!(user_after_deletion.unwrap().is_none());
        }
    }

    #[tokio::test]
    async fn test_get_user_by_email_success() {
        let pool = CassandraPool::new("cassandra.int.butterhead.net").await.unwrap();
        let conn = get_connection(&pool).await.unwrap();
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserGET_USER_BY_EMAIL_SUCCESS@example.com".to_string(),
            password_hash: Some("password123".to_string()),
            description: Some("Some long description".to_string()),
            interests: Vec::new(),
            created_at: None,
            updated_at: None,
            last_login: None,
        };

        match delete_user(conn, &user.user_id, &user.email).await {
            Ok(_) => (),
            Err(e) => {
                panic!("delete_user error: {:?}", e);
            }
        }

        let create_result = create_user(conn, user.clone()).await;
        if let Err(e) = create_result {
            println!("create_result: {:?}", e);
        }
        // Act: get the user by email
        let result = get_user_by_email(conn, &user.email).await;
        let result = result.unwrap();
        assert!(result.is_some());
        let retrieved_user = result.unwrap();
        assert_eq!(retrieved_user.email, user.email);
    }

    #[tokio::test]
    async fn test_verify_password() {
        let password = "password123";
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        let hashed_password_2 = hash(password, DEFAULT_COST).unwrap();

        println!(
            "password: {}, hashed_password: {}",
            password, hashed_password
        );
        println!(
            "password: {}, hashed_password_2: {}",
            password, hashed_password_2
        );
        assert!(verify(password, &hashed_password).unwrap());
        assert!(!verify("wrongpassword", &hashed_password).unwrap());
    }
}
