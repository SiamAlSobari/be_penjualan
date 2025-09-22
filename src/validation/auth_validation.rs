use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct RegisterUser {
    #[validate(length(min = 3, message = "Minimal 3 karakter"))]
    pub user_name: String,

    #[validate(email(message = "Format email tidak valid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Minimal 6 karakter"))]
    pub password: String,
}
