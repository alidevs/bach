use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}
