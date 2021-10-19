use crate::error::CustomError;
use actix_web::{web, HttpResponse, Responder};

pub async fn api() -> impl Responder {
  HttpResponse::Ok().body(format!("Welcome to API"))
}
