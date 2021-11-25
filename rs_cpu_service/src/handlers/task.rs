use crate::error::CustomError;
use actix_web::{get, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::env;

pub fn fibonacci_reccursive(n: usize) -> u64 {
  if n < 0 {
    panic!("{} is negative!", n);
  }
  match n {
    0 => panic!("zero is not a right argument to fibonacci_reccursive()!"),
    1 | 2 => 1,
    3 => 2,
    _ => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2),
  }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Fibo {
  #[serde(default)]
  pub num: usize,
}

pub async fn task(query: web::Query<Fibo>) -> Result<HttpResponse, CustomError> {
  let res = fibonacci_reccursive(query.num);
  Ok(HttpResponse::Ok().body(res.to_string()))
}
