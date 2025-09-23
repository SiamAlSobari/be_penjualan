use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize,Deserialize)]
pub struct ResponseFindByEmail {
    pub id: String,
    pub email: String,
    pub hash_password:String
}