#![allow(clippy::extra_unused_lifetimes)]

use diesel::{r2d2::ConnectionManager, PgConnection};
use serde::{Deserialize, Serialize};

// use crate::schema::users::dsl::*;
use super::schema::*;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
#[derive(Debug, Serialize, Deserialize, Queryable,Insertable)]
#[diesel(table_name = users)]
pub struct User{
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at:  chrono::NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SlimUser{
    pub first_name: String,
    pub last_name: String,
    pub email: String
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUser{
    pub first_name: String,
    pub email: String,
    pub token: String
}
impl From<User> for SlimUser{
    fn from(user: User) -> Self {
        SlimUser{
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email
        }
    }
}
#[derive(Insertable, Debug)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub password: &'a str,
    pub email: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

