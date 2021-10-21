use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::io::Result;

#[macro_use]
extern crate magic_crypt;

mod auth_validator;
mod cookie;
mod db;
mod error;
mod handlers;
mod models;
mod request;
mod state;
mod token;

use auth_validator::bearer_auth_validator;
use db::db_connect;
use error::CustomError;
use handlers::api::api;
use handlers::authenticate::authenticate;
use handlers::callback::callback;
use handlers::discovery::discovery;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::login::login;
use handlers::token::token;
use handlers::token_pass::token_pass;
use state::AppState;

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
            .service(
                web::scope("/local")
                    .wrap(HttpAuthentication::bearer(bearer_auth_validator))
                    .route("/callback", web::get().to(callback))
                    .route("/api", web::post().to(api)),
            )
            .service(
                web::scope("/oauth")
                    .route("/auth", web::post().to(authenticate))
                    .route("/token", web::post().to(token))
                    .route("/tokenpass", web::post().to(token_pass))
                    .route("/identity", web::post().to(find_user))
                    .route("/login", web::get().to(login))
                    .route(
                        "/.well-known/openid-configuration",
                        web::get().to(discovery),
                    ),
            )
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
