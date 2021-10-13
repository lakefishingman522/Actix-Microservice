use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use reddb::RonDb;

mod cookie;
mod error;
mod handlers;
mod models;
mod request;
mod state;
mod token;

use handlers::authenticate::auth;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::local_server::local_server;
use handlers::login::login;
use models::User;
use state::AppState;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .data(AppState {
                app_name: String::from("Rust SSO"),
                db: RonDb::new::<User>("users.db").unwrap(),
                private_key: token::get_key(),
            })
            .wrap(Logger::default())
            .service(index)
            .service(local_server)
            .route("/auth", web::get().to(auth))
            .route("/login", web::post().to(login))
            .route("/identity", web::post().to(find_user))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
