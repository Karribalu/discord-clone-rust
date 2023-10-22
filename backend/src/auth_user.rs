use crate::errors::ServiceError;
use crate::models::{LoggedUser, Pool, SlimUser, User};
use crate::schema::users::dsl::users;
use crate::schema::users::email;
use crate::utils::{create_token, decode_token, verify};

use actix_web::{web, HttpRequest, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct LoginData {
    pub email: String,
    pub password: String,
}

pub async fn login_user(
    user_data: web::Json<LoginData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    let user = web::block(move || login(user_data.into_inner(), pool)).await??;
    let jwt_token = create_token(&user)?;
    let logged_user = LoggedUser {
        first_name: user.first_name.into(),
        email: user.email.into(),
        token: jwt_token,
    };
    Ok(HttpResponse::Ok().json(logged_user))
}
pub async fn login_user_jwt(request: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    let auth_header = request.headers().get("Authorization");
    if let Some(auth_header) = auth_header {
        if let Ok(auth_header_str) = auth_header.to_str() {
            if auth_header_str.starts_with("Bearer") {
                let token = &auth_header_str[7..];
                return Ok(HttpResponse::Ok().json(login_jwt(token)?));
            }
        }
    }
    Err(ServiceError::Unauthorized)?
}
fn login(user_data: LoginData, pool: web::Data<Pool>) -> Result<SlimUser, ServiceError> {
    let conn = pool.get().unwrap();
    let mut items = users
        .filter(email.eq(&user_data.email))
        .load::<User>(&conn)?;
    if let Some(user) = items.pop() {
        if let Ok(matching) = verify(&user.password, &user_data.password) {
            if matching {
                return Ok(user.into());
            }
        }
    }
    Err(ServiceError::Unauthorized)
}

fn login_jwt(token: &str) -> Result<SlimUser, ServiceError> {
    decode_token(&token)
}
