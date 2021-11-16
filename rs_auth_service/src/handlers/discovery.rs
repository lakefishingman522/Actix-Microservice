use actix_web::{HttpResponse, Responder};

pub async fn discovery() -> impl Responder {
  HttpResponse::Ok().body(format!("Welcome"))
}
