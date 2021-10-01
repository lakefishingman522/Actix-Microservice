use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod client;
mod handlers;
mod jwt;
mod state;
mod user;

use handlers::auth::auth;
use handlers::identity::find_user;
use handlers::index::index;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(index))
            .route("/identity", web::post().to(find_user))
            .route("/auth", web::post().to(auth))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
