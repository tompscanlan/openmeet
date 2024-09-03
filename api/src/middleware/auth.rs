// src/middleware/auth.rs
use rocket::request::{self, FromRequest};
use rocket::outcome::Outcome;
use rocket::http::Status;
use rocket::Request;

pub struct AuthToken(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        if let Some(token) = request.headers().get_one("Authorization") {
            if verify_token(token).is_ok() {
                return Outcome::Success(AuthToken(token.to_string()));
            }
        }
        
        Outcome::Error((Status::Unauthorized, ()))
    }
}

fn verify_token(token: &str) -> Result<(), &'static str> {
    println!("Verifying token: {}", token);

    Ok(())
}