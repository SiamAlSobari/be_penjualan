use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Profile {
    pub user_name: String,
    pub user_id: String,
    pub avatar_url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}