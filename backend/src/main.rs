#[macro_use]
extern crate diesel;
mod auth_user;
mod errors;
mod models;
mod register_handler;
mod schema;
mod utils;

pub use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var(
        "RUST_LOG",
        "crud-app-rust=debug,actix_web=info,actix_server=info",
    );
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    //create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: models::Pool = r2d2::Pool::builder()
        .max_size(2)
        .build(manager)
        .expect("Failed to create pool.");
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    //start http server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(time::Duration::days(1))
                    .secure(false), // only true when you have https
            ))
            .app_data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/register")
                            .route(web::post().to(register_handler::register_user)),
                    )
                    .service(web::resource("/login").route(web::post().to(auth_user::login_user))),
            )
            .service(web::scope("/authenticate").service(
                web::resource("/jwt-token").route(web::post().to(auth_user::login_user_jwt)),
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
