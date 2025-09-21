use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;


#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String
}

#[derive(Debug,Deserialize,Serialize,FromRow)]
pub struct Profile {
    pub username : String,
    pub user_id : String,
    pub avatar_url : String
}
