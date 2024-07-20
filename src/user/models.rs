use crate::schema::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use diesel::prelude::*;
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Insertable, Clone, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl NewUser {
    pub fn new(
        username: String,
        pass: String,
        first_name: String,
        last_name: String,
        email: String,
    ) -> NewUser {
        let id = Uuid::new_v4().to_string();
        let hashed_password: String = hash(pass, DEFAULT_COST).unwrap();

        let timestamp = chrono::Local::now().naive_local();

        NewUser {
            id,
            username,
            password: hashed_password,
            first_name,
            last_name,
            email,
            timestamp,
        }
    }
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl User {
    pub fn verify_password(&self, pass: String) -> bool {
        verify(pass, &self.password).unwrap()
    }
}
