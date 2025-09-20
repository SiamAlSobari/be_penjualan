use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize,Serialize,Validate)]
pub struct RegisterUser {

    #[validate(length(min= 3 , message ="Minimal 3 karakter"))]
    username: String,

    #[validate(email(message = "Format email tidak valid"))]
    email: String
}