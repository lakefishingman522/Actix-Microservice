use actix_web::dev::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use futures::future::FutureExt;
use prometheus::{labels, opts, register_counter, register_gauge, register_histogram_vec};
use prometheus::{Counter, Encoder, Gauge, HistogramVec, TextEncoder};

use std::io::Result;

#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate lazy_static;

mod auth_validator;
mod cookie;
mod db;
mod error;
mod handlers;
mod metrics;
mod models;
mod request;
mod state;
mod token;

use db::db_connect;
use error::CustomError;
use handlers::index::index;
use handlers::metrics::metrics;
use handlers::task::task;
use state::AppState;
use std::env;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
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
            .service(index)
            .route("/metrics", web::get().to(metrics))
            .service(web::scope("/cpu").route("/task", web::get().to(task)))
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
