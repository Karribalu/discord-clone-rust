use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use diesel::result::Error as DBError;
#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "Bad Request {}", _0)]
    BadRequest(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server error, Please try again")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
impl From<r2d2::Error> for ServiceError {
    fn from(_: r2d2::Error) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<DBError> for ServiceError {
    fn from(error: DBError) -> ServiceError {
        match error {
            DBError::DatabaseError(_kind, _info) => ServiceError::InternalServerError,
            _ => ServiceError::InternalServerError,
        }
    }
}
