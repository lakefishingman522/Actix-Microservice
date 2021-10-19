use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;

mod cookie;
mod db;
mod error;
mod handlers;
mod models;
mod request;
mod state;
mod token;

use db::db_connect;
use error::CustomError;
use handlers::authenticate::auth;
use handlers::callback::callback;
use handlers::discovery::discovery;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::login::login;
use models::User;
use state::AppState;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let mongodb: mongodb::Client = db_connect()
        .await
        .map_err(|_e| CustomError::NoDbConnection)
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from("Rust SSO"),
                db: mongodb.clone(),
                private_key: token::get_key(),
            })
            .wrap(Logger::default())
            .service(index)
            .service(web::scope("/local").route("/callback", web::get().to(callback)))
            .service(
                web::scope("/oauth")
                    .route("/auth", web::get().to(auth))
                    .route(
                        "/.well-known/openid-configuration",
                        web::get().to(discovery),
                    )
                    .route("/identity", web::post().to(find_user))
                    .route("/login", web::post().to(login)),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
