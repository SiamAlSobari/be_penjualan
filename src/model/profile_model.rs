use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct ResponseFindByUserName {
    pub id: String,
    pub user_name: String
}