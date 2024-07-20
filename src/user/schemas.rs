use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub password: String,
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

#[derive(Serialize, Deserialize)]
pub struct ListUsersResponse {
    pub users: Vec<UserSchema>,
}

#[derive(Serialize, Deserialize)]
pub struct UserSchema {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl From<crate::user::models::User> for UserSchema {
    fn from(user: crate::user::models::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
            timestamp: user.timestamp,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}
