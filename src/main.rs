use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod handlers;
mod state;

use handlers::auth::auth;
use handlers::index::index;
use std::io::Result;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/auth").route(web::post().to(auth)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
