use crate::errors::ServiceError;
use crate::models::SlimUser;
use argon2::Config;
use chrono::{Duration, Local};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sha2::Sha384;
use std::default::Default;

pub static SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(16)));
const SALT: &'static [u8] = b"bala'ssupersecuresalt";
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    // issuer
    iss: String,
    // subject
    sub: String,
    //issued at
    iat: i64,
    // expiry
    exp: i64,
    first_name: String,
    last_name: String,
    // user email
    email: String,
}

impl Claims {
    fn with_email(email: &str, first_name: &str, last_name: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            first_name: first_name.to_owned(),
            last_name: last_name.to_owned(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::seconds(60)).timestamp(),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), &SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

pub fn create_token(data: &SlimUser) -> Result<String, ServiceError> {
    let claims = Claims::with_email(
        data.email.as_str(),
        data.first_name.as_str(),
        data.last_name.as_str(),
    );

    claims
        .sign_with_key(&get_key())
        .map_err(|_| ServiceError::InternalServerError)
}

pub fn decode_token(token: &str) -> Result<SlimUser, ServiceError> {
    token
        .verify_with_key(&get_key())
        .map(|data: SlimUser| Ok(data.into()))
        .map_err(|_| ServiceError::Unauthorized)?
}

fn get_key() -> Hmac<Sha384> {
    Hmac::new_from_slice(b"some-secret").unwrap()
}
