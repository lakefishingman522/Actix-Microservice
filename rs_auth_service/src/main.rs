use actix_web::dev::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use futures::future::FutureExt;
use std::env;
use std::io::Result;

#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate lazy_static;

mod db;
mod error;
mod handlers;
mod helpers;
mod models;

use crate::helpers::{metrics, token};
use db::mongo;
use error::CustomError;
use handlers::authenticate::authenticate;
use handlers::discovery::discovery;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::login::login;
use handlers::metrics::metrics;
use handlers::token::token;
use handlers::token_pass::token_pass;
use models::app_state::AppState;
#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let port = env::var("APP_PORT").unwrap();
    let app_name = env::var("APP_NAME").unwrap();
    let mongodb: mongodb::Client = mongo::connect()
        .await
        .map_err(|_e| CustomError::NoDbConnection)
        .unwrap();

    println!(
        "[Server] Launching {:} on port {:?} !",
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
            .wrap_fn(|req, srv| {
                metrics::HTTP_COUNTER.inc();
                let timer = metrics::HTTP_REQ_HISTOGRAM
                    .with_label_values(&["env"])
                    .start_timer();
                srv.call(req).map(|res| {
                    timer.observe_duration();
                    res
                })
            })
            .route("/", web::get().to(index))
            .route("/metrics", web::get().to(metrics))
            .service(
                web::scope("/oauth")
                    .route("/", web::get().to(index))
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
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
