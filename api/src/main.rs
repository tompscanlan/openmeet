use rocket::serde::{json::Json};
use rocket::http::Status;
use rocket::{post, get, launch, routes};
use jsonwebtoken::{encode, Header, EncodingKey, DecodingKey, Validation, TokenData};
use bcrypt::{hash, verify};
use std::env;
use uuid::Uuid;
mod user;
use crate::user::{create_user, get_user_by_email, User, Claims};
use cassandra_cpp::Cluster;

#[post("/register", data = "<user>")]
async fn register(user: Json<User>) -> Result<Json<String>, Status> {
    let hashed_password = hash(&user.password_hash, 10).map_err(|_| Status::InternalServerError)?;

    let new_user = User {
        user_id: Uuid::new_v4(),
        username: user.username.clone(),
        email: user.email.clone(),
        password_hash: hashed_password,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_login: chrono::Utc::now(),
    };

    create_user(new_user).await.map_err(|_| Status::InternalServerError)?;

    Ok(Json(format!("User {} registered successfully", user.email)))
}

#[post("/login", data = "<user>")]
async fn login(user: Json<User>) -> Result<Json<String>, Status> {
    if let Some(extant_user) = get_user_by_email(&user.email).await {
        if verify(&user.password_hash, &extant_user.password_hash).unwrap() {
            let claims = Claims {
                sub: user.email.clone(),
                exp: 10000000000, // Set expiration time
            };
            let token = encode(&Header::default(), &claims, &EncodingKey::from_secret("secret".as_ref()))
                .map_err(|_| Status::InternalServerError)?;
            return Ok(Json(token));
        } else {
            return Err(Status::Unauthorized);
        }
    } else {
        return Err(Status::Unauthorized);
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
    cluster.set_contact_points(&contact_points)
        .map_err(|e| format!("Failed to set contact points: {}", e))?;
    
    let username = env::var("CASSANDRA_USERNAME").unwrap_or_default();
    let password = env::var("CASSANDRA_PASSWORD").unwrap_or_default();

    if !username.is_empty() && !password.is_empty() {
        cluster.set_credentials(&username, &password)
            .map_err(|e| format!("Failed to set credentials: {}", e))?;
    }
    
    Ok(cluster)
}