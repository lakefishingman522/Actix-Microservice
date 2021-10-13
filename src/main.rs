use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use reddb::RonDb;

mod cookie;
mod error;
mod handlers;
mod jwt;
mod request;
mod state;
//mod templates;
mod user;

use handlers::authenticate::auth;
use handlers::identity_provider::find_user;
use handlers::index::index;
use handlers::signin::signin;
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
            .service(index)
            .route("/auth", web::get().to(auth))
            .route("/signin", web::post().to(signin))
            .route("/identity", web::post().to(find_user))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
