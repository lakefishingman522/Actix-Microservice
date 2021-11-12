use actix_web::{get, web, HttpResponse, Responder};

use crate::state::AppState;

#[get("/")]
pub async fn index(state: web::Data<AppState>) -> impl Responder {
  let app_name = &state.app_name;
  HttpResponse::Ok().body(format!("Welcome to {}!", app_name))
}
