use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
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
use handlers::authenticate::authenticate;
use handlers::discovery::discovery;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::login::login;
use handlers::token::token;
use handlers::token_pass::token_pass;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    let port = env::var("APP_PORT").unwrap();
    let app_name = env::var("APP_NAME").unwrap();
    let mongodb: mongodb::Client = db_connect()
        .await
        .map_err(|_e| CustomError::NoDbConnection)
        .unwrap();

    println!(
        "[Server] Launching {:} on port {:?}",
        app_name,
        port.clone()
    );
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from(&app_name),
                db: mongodb.clone(),
                private_key: token::get_key(),
            })
            .wrap(Logger::default())
            .service(index)
            .route("/auth", web::post().to(authenticate))
            .route("/token", web::post().to(token))
            .route("/tokenpass", web::post().to(token_pass))
            .route("/identity", web::post().to(find_user))
            .route("/login", web::get().to(login))
            .route(
                "/.well-known/openid-configuration",
                web::get().to(discovery),
            )
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
