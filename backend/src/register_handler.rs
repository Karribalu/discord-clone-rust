use crate::errors::ServiceError;
use crate::models::{NewUser, Pool, SlimUser, User};
use crate::schema::users::dsl::users;
use crate::utils::hash_password;
use actix_web::{web, HttpResponse};
use diesel::{insert_into, RunQueryDsl};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
pub async fn register_user(
    user_data: web::Json<UserData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || register_query(user_data.into_inner(), pool)).await??;
    Ok(HttpResponse::Ok().json(&user))
}
fn register_query(user_data: UserData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    let conn = pool.get().unwrap();
    let hashed_password = hash_password(&user_data.password)?;

    let new_user = NewUser {
        first_name: &user_data.first_name,
        last_name: &user_data.last_name,
        password: &hashed_password,
        email: &user_data.email,
        created_at: chrono::Local::now().naive_local(),
    };
    let inserted_user: User = insert_into(users).values(&new_user).get_result(&conn)?;
    dbg!(&inserted_user);

    return Ok(inserted_user.into());
}
