use std::env;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use validator::ValidationErrors;

#[derive(Deserialize, Serialize)]
pub struct Response<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ValidateErrItem {
    field: String,
    code: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject (misal user_id)
    exp: i64,    // expiry timestamp
}
pub fn generate_jwt(user_id: &str) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .ok_or("Gagal hitung expiry")? // propagate Err kalau None
        .timestamp();

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };
    let secret = env::var("SECRET").expect("ENV ERR");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| e.to_string())
}

pub fn verify_jwt(token: &str) -> Result<Claims, String> {
    let secret = "my_secret"; // sama dengan generate
    let validation = Validation::default();

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims),
        Err(err) => Err(format!("Token tidak valid: {}", err)),
    }
}


pub fn map_validation(err: ValidationErrors) -> Vec<ValidateErrItem> {
    err.field_errors()
        .iter()
        .flat_map(|(field, erros)| {
            erros.iter().map(|e| ValidateErrItem {
                field: field.to_string(),
                code: e.code.to_string(),
            })
        })
        .collect::<Vec<ValidateErrItem>>()
}
