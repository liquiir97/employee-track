use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Employee {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub age: u8,
    pub telephone: String,
    pub diploma: Option<Vec<String>>,
    pub password: Option<String>,
    pub global_handler: Option<String>
}