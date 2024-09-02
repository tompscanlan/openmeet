use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, post, delete, routes};
use std::env;
use uuid::Uuid;
mod user;
use crate::user::{create_user, User, UserLogin, UserRegister, delete_user, get_all_users, get_user_by_id};
use cassandra_cpp::Cluster;
use serde_json::json;
mod middleware;
use crate::middleware::auth::AuthToken;

#[derive(Serialize)]
struct SuccessResponse {
    message: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

#[derive(Serialize)]
struct UserReset {
    email: String,
    old_password: String,
    new_password: String,
}
use chrono::Utc;

#[post("/register", data = "<user_register>")]
async fn register(user_register: Json<UserRegister>) -> Result<Json<SuccessResponse>, Status> {
    let user_register = user_register.into_inner();

    let now = Utc::now().timestamp();
    let new_user = User {
        user_id: Uuid::new_v4(),
        username: user_register.username.clone(),
        email: user_register.email.clone(),
        password_hash: user_register.password.clone(),
        created_at: now,
        updated_at: now,
        last_login: 0,
    };

    create_user(new_user)
        .await
        .map_err(|e| {
            eprintln!("Failed to create user: {}", e);
            Status::InternalServerError
        })?;

    Ok(Json(SuccessResponse {
        message: format!("User {} registered successfully", user_register.email),
    }))
}


#[delete("/users/<user_id>")]
async fn frontend_delete_user(_auth: AuthToken,user_id: &str) -> Result<Json<SuccessResponse>, Status> {
    let user_id = Uuid::parse_str(user_id).map_err(|e| {
        eprintln!("Invalid UUID: {}", e);
        Status::BadRequest
    })?;

    let user = get_user_by_id(user_id).await;
    if user.is_none() {
        return Err(Status::NotFound);
    }
    let user = user.unwrap();

    let result = delete_user(&user_id, &user.email).await;
    match result {
        Ok(_) => Ok(Json(SuccessResponse {
            message: "User deleted successfully".to_string(),
        })),
        Err(e) => {
            eprintln!("Failed to delete user: {}", e);
            return Err(Status::InternalServerError);
        }
    }
}
#[get("/users")]
async fn list_users(_auth: AuthToken) -> Result<Json<Vec<User>>, Status> {
    match get_all_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("Failed to retrieve users: {}", e);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/login", data = "<user_login>")]
async fn frontend_login(user_login: Json<UserLogin>) -> Json<serde_json::Value> {
    let user = user_login.into_inner();
        match user::login(&user.email, &user.password).await {
            Ok(token) => {
                println!("token: {:?}", token);
                Json(json!({ "success": true, "message": "Login successful", "token": token }))
            }
            Err(e) => {
                eprintln!("Failed to generate token: {}", e);
                Json(json!({ "success": false, "message": "Failed to generate token" }))
            }
        }
    }
    // #[post("/reset_password", data = "<user_reset>")]
    // async fn reset_password(user_reset: Json<UserReset>) -> Result<Json<SuccessResponse>, Status> {
    //     let user_reset = user_reset.into_inner();
    //     let user = user::get_user_by_email(&user_reset.email).await;
    
    //     if user.is_none() {
    //         return Err(Status::NotFound);
    //     }
    
    //     let mut user = user.unwrap();
    //     user.password_hash = hash(&user_reset.new_password, DEFAULT_COST).map_err(|e| e.to_string())?;
        
    //     // Update user in the database (you'll need to implement this)
    //     // update_user(&user).await?;
    
    //     Ok(Json(SuccessResponse {
    //         message: "Password reset successfully".to_string(),
    //     }))
    // }

    
#[get("/")]
fn index() -> &'static str {
    "Welcome to the API"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, register, frontend_login, list_users, frontend_delete_user])
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
