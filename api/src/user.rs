use crate::init_cluster;
use bcrypt::{hash, verify, DEFAULT_COST};
use cassandra_cpp::BindRustType;
use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::Json;
use cassandra_cpp::LendingIterator;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub last_login: i64,
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
    statement.bind(4, now).map_err(|e| e.to_string())?;
    statement.bind(5, now).map_err(|e| e.to_string())?;
    statement.bind(6, now).map_err(|e| e.to_string())?;

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
pub async fn create_user(original_user: User) -> Result<User, String> {
    // let cloned_original_user: User = original_user.clone();
    let mut user = original_user;

    // create bcrypted password_hash
    user.password_hash = hash(&user.password_hash, DEFAULT_COST).map_err(|e| e.to_string())?;

    // create uuid
    user.user_id = Uuid::new_v4();

    // check email is valid
    if !is_valid_email(&user.email).await {
        return Err("Invalid email".to_string());
    }

    // time now
    let now = Utc::now().timestamp_millis();

    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    if check_email_exists(&session, &user.email).await? {
        return Err("Email already exists".to_string());
    }
    insert_user(&session, &user, now).await?;
    insert_email_index(&session, &user.email, user.user_id).await?;

    Ok(user)
}

pub async fn get_user_by_id(user_id: Uuid) -> Option<User> {

    let mut cluster = init_cluster().await.unwrap();
    let session = cluster.connect().await.unwrap();

    let query = "SELECT * FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, user_id).unwrap();

    let result = statement.execute().await.unwrap();
    let row = result.first_row()?;

    Some(User {
        user_id: row.get_column_by_name("user_id").unwrap().get_uuid().unwrap().into(),
        username: row.get_column_by_name("username").unwrap().get_string().unwrap(),
        email: row.get_column_by_name("email").unwrap().get_string().unwrap(),
        password_hash: row.get_column_by_name("password_hash").unwrap().get_string().unwrap(),
        created_at: row.get_column_by_name("created_at").unwrap().get_i64().unwrap(),
        updated_at: row.get_column_by_name("updated_at").unwrap().get_i64().unwrap(),
        last_login: row.get_column_by_name("last_login").unwrap().get_i64().unwrap(),
    })
}

pub async fn get_user_by_email(email: &str) -> Option<User> {
    let mut cluster = init_cluster().await.unwrap();
    let session = cluster.connect().await.unwrap();

    let query = "SELECT * FROM openmeet.users WHERE email = ?";
    let mut statement = session.statement(query);
    statement.bind(0, email).unwrap();

    let result = statement.execute().await.unwrap();
    let row = result.first_row()?;

    Some(User {
        user_id: row
            .get_column_by_name("user_id")
            .ok()?
            .get_uuid()
            .ok()?
            .into(),
        username: row.get_column_by_name("username").ok()?.get_string().ok()?,
        email: row.get_column_by_name("email").ok()?.get_string().ok()?,
        password_hash: row
            .get_column_by_name("password_hash")
            .ok()?
            .get_string()
            .ok()?,
        created_at: row
            .get_column_by_name("created_at")
            .ok()?
            .get_i64()
            .unwrap_or_default(),
        updated_at: row
            .get_column_by_name("updated_at")
            .ok()?
            .get_i64()
            .unwrap_or_default(),
        last_login: row
            .get_column_by_name("last_login")
            .ok()?
            .get_i64()
            .unwrap_or_default(),
    })
}


fn generate_token(user: &User) -> Result<String, String> {
    let claims = Claims {
        sub: user.user_id.to_string(),
        exp: (Utc::now().timestamp() + 3600) as usize, // Token valid for 1 hour
    };

    let encoding_key = EncodingKey::from_secret("your_secret_key".as_ref());
    encode(&Header::default(), &claims, &encoding_key).map_err(|e| e.to_string())
}

pub async fn login(email: &str, password: &str) -> Result<String, String> {
    if let Some(user) = get_user_by_email(email).await {
        let password_verified = verify(password, &user.password_hash);

        match password_verified {
            Ok(true) => {
                let token = generate_token(&user)?; 
                Ok(token)
            }
            Ok(false) => Err("Invalid credentials".to_string()),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Err("User not found".to_string())
    }
}

pub async fn delete_user(user_id: &Uuid, email: &str) -> Result<(), String> {
    let mut cluster = init_cluster().await?;
    let session = cluster.connect().await.map_err(|e| e.to_string())?;

    let user = get_user_by_email(email).await;
    if user.is_none() {
        return Err("User not found".to_string());
    }
    // let user = user.unwrap();

    let query = "DELETE FROM openmeet.users WHERE user_id = ?";
    let mut statement = session.statement(query);
    statement.bind(0, *user_id).map_err(|e| e.to_string())?;
    statement.execute().await.map_err(|e| e.to_string())?;

    // also delete from email_index
    let query = "DELETE FROM openmeet.email_index WHERE email = ?";
    let mut statement = session.statement(query);
    statement.bind(0, email).map_err(|e| e.to_string())?;
    statement.execute().await.map_err(|e| e.to_string())?;

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
        users.push(User {
            user_id: row.get_column_by_name("user_id").ok()
                .and_then(|col| col.get_uuid().ok())
                .ok_or("Invalid user_id".to_string())?
                .into(),
                username: row.get_column_by_name("username").ok()
                .and_then(|col| col.get_string().ok())
                .ok_or("Invalid username".to_string())?, // Convert Option to Result
            email: row.get_column_by_name("email").ok()
                .and_then(|col| col.get_string().ok())
                .ok_or("Invalid email".to_string())?, 
            password_hash: row.get_column_by_name("password_hash").ok()
                .and_then(|col| col.get_string().ok())
                .ok_or("Invalid password_hash".to_string())?,
            created_at: row.get_column_by_name("created_at").ok()
                .and_then(|col| col.get_i64().ok())
                .unwrap_or_default(),
            updated_at: row.get_column_by_name("updated_at").ok()
                .and_then(|col| col.get_i64().ok())
                .unwrap_or_default(),
            last_login: row.get_column_by_name("last_login").ok()
                .and_then(|col| col.get_i64().ok())
                .unwrap_or_default(),
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
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserLOGIN_SUCCESS@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: 0,
            updated_at: 0,
            last_login: 0,
        };

        let created_user = create_user(user.clone()).await;

        if let Err(e) = created_user {
            if !e.to_string().contains("Email already exists") {
                panic!("Failed to create user: {}", e);
            }
        }
        // Act: attempt to login with correct credentials
        let result = login(&user.email, &user.password_hash).await;

        if let Err(e) = result {
            println!("result--->: {:?}", e);
        } else {
            let token = result.unwrap().clone();
            assert_eq!(token.len() > 0, true);
        }
    }

    #[tokio::test]
    async fn test_login_credentials_create_user() {

    // Setup: create a user instance
    let user = User {
        user_id: Uuid::new_v4(),
        username: "testuser".to_string(),
        email: "testuserCREATE_LOGIN@example.com".to_string(),
        password_hash: "password123".to_string(),
        created_at: Utc::now().timestamp_millis(),
        updated_at: Utc::now().timestamp_millis(),
        last_login: Utc::now().timestamp_millis(),
    };

    //  delete any user with this email
    let _ = delete_user(&user.user_id, &user.email).await;

    // Act: create the user
    let create_result = create_user(user.clone()).await;
    assert!(create_result.is_ok());

    // Act: attempt to login with the same user
    let login_result = login(&user.email, &user.password_hash).await;

    // Assert: check that login was successful
    assert!(login_result.is_ok());
    let token = login_result.unwrap();
    assert!(!token.is_empty());
    }

    #[tokio::test]
    async fn test_login_second_layer() {
        // in a loop
        // register a user with a random email and password
        // login with the user
        // delete the user
    for i in 0..5 {
        // Generate a random email and password
        let email = format!("testuser{}@example.com", i);
        let password = format!("password{}", i);

        let user = get_user_by_email(email.as_str()).await;
        if let Some(user) = user {
            let _ = delete_user(&user.user_id, &user.email).await;
        }

        // Setup: create a user instance
        let user = User {
            user_id: Uuid::new_v4(),
            username: format!("testuser{}", i),
            email: email.clone(),
            password_hash: password.clone(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
        };

        // Act: create the user
        let create_result = crate::register(Json(
            UserRegister {
                username: user.username.clone(),
                email: email.clone(),
                password: password.clone(),
            }
        )).await;
        assert!(create_result.is_ok());

        // Act: attempt to login with the same user
        let login_result = crate::frontend_login(Json(
            UserLogin {
                email: email.clone(),
                password: password.clone(),
            }
        )).await;
              
        let token = login_result.into_inner();
        // Assert: check that login was successful
        assert!(!token.is_string(), "Token should be a string");
        // Act: delete the user
        let delete_result = delete_user(&user.user_id, &user.email).await;
        assert!(delete_result.is_ok());
    }

        
    }

    #[tokio::test]
    async fn test_login_failure_wrong_password() {
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuser@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: 0,
            updated_at: 0,
            last_login: 0,
        };

        // ignore any duplicate errors
        let _ = create_user(user.clone()).await;

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
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
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
        // Setup: create a user instance

        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserDUPLICATE@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
        };

        // Act: create the user
        let result = create_user(user.clone()).await;

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
            password_hash: "password456".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
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
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
        };

        // Act: attempt to create the user
        let result = create_user(user).await;

        // Assert: check that user creation failed due to invalid email
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_user_success() {
        // Setup: create a user and insert into the database
        let random_email = format!("{uuid}@example.com", uuid = Uuid::new_v4());
        let user_sample = User {
            user_id: Uuid::new_v4(),
            username: random_email.clone(),
            email: random_email.clone(),
            password_hash: "password123".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
        };

        let create_result = create_user(user_sample.clone()).await;
        assert!(create_result.is_ok());

        let user_id = create_result.unwrap().user_id;

        // Act: delete the user
        let delete_result = delete_user(&user_id, &user_sample.email).await;
        if let Err(e) = delete_result.clone() {
            println!("delete_result: {:?}", e);
        }
        // Assert: check that user deletion was successful
        assert!(delete_result.is_ok(), "User deletion should succeed");

        // Verify that the user no longer exists
        let user_after_deletion = get_user_by_email(&user_sample.email).await;
        assert!(
            user_after_deletion.is_none(),
            "User should not exist after deletion"
        );
    }

    #[tokio::test]
    async fn test_get_user_by_email_success() {
        // Setup: create a user and insert into the database
        let user = User {
            user_id: Uuid::new_v4(),
            username: "testuser".to_string(),
            email: "testuserGET_USER_BY_EMAIL_SUCCESS@example.com".to_string(),
            password_hash: "password123".to_string(),
            created_at: Utc::now().timestamp_millis(),
            updated_at: Utc::now().timestamp_millis(),
            last_login: Utc::now().timestamp_millis(),
        };

        let create_result = create_user(user.clone()).await;
        if let Err(e) = create_result {
            println!("create_result: {:?}", e);
        }
        // Act: get the user by email
        let result = get_user_by_email(&user.email).await;
        assert!(result.is_some());
        let retrieved_user = result.unwrap();
        assert_eq!(retrieved_user.email, user.email);
    }
    
    #[tokio::test]
    async fn test_verify_password() {
        let password = "password123";
        let hashed_password = hash(password, DEFAULT_COST).unwrap();
        let hashed_password_2 = hash(password, DEFAULT_COST).unwrap();

        println!("password: {}, hashed_password: {}", password, hashed_password);
        println!("password: {}, hashed_password_2: {}", password, hashed_password_2);
        assert!(verify(password, &hashed_password).unwrap());
        assert!(!verify("wrongpassword", &hashed_password).unwrap());
    }
}
