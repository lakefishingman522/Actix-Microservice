use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use reddb::RonDb;

mod cookie;
mod handlers;
mod jwt;
mod request;
mod state;
mod user;

use handlers::authenticate::authenticate;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::login::login;
use state::AppState;
use std::io::Result;
use user::User;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Rust SSO"),
                db: RonDb::new::<User>("users.db").unwrap(),
            })
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/login", web::post().to(login))
            .route("/identity", web::post().to(find_user))
            .route("/authenticate", web::get().to(authenticate))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
