use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response<T> {
    pub status : String,
    pub message: String,
    pub data: Option<T>
}