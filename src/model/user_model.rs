use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseFindByEmail {
    pub id: String,
    pub email: String,
}