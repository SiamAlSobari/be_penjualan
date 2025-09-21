use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct Response<T> {
    pub status : String,
    pub message: String,
    pub data: Option<T>
}