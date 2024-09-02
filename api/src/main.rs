use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::{get, launch, post, routes};
use std::env;
use uuid::Uuid;
mod user;
use crate::user::{create_user, User, UserLogin, UserRegister};
use cassandra_cpp::Cluster;
use serde_json::json;
use bcrypt::DEFAULT_COST;
use bcrypt::hash;

#[derive(Serialize)]
struct SuccessResponse {
    message: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
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

#[post("/login", data = "<user_login>")]
async fn login(user_login: Json<UserLogin>) -> Json<serde_json::Value> {
    let user = user_login.into_inner();
    println!("received /login request for user {}, password {}", user.email, user.password);
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

#[get("/")]
fn index() -> &'static str {
    "Welcome to the API"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, register, login])
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
