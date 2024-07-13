use crate::schema::*;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub email: &'a str,
    pub timestamp: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct NewUserOwned {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}

impl NewUserOwned {
    pub fn as_new_user(&self) -> NewUser {
        NewUser {
            id: &self.id,
            username: &self.username,
            first_name: &self.first_name,
            last_name: &self.last_name,
            email: &self.email,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub timestamp: chrono::NaiveDateTime,
}
