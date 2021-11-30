use actix_web::dev::Service;
use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use futures::future::FutureExt;

use std::io::Result;

#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate lazy_static;

mod error;
mod handlers;
mod metrics;
mod models;
mod request;
mod state;

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
    println!(
        "[Server] Launching {:} on port {:?}",
        app_name,
        port.clone()
    );
    HttpServer::new(move || {
        App::new()
            .data(AppState {
                app_name: String::from(&app_name),
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
                web::scope("/cpu")
                    .route("/", web::get().to(index))
                    .route("/task", web::get().to(task)),
            )
    })
    .bind(["0.0.0.0:", &port].concat())
    .expect("Can't launch server")
    .run()
    .await
}
