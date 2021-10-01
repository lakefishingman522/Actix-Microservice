use actix_rt::System;
use actix_web::client::{Client, ClientResponse, JsonPayloadError, PayloadError};
use actix_web::dev::Decompress;
use actix_web::Error;
use std::collections::HashMap;

use crate::user::User;
use std::env;

pub async fn request(endpoint: &str) -> Result<User, JsonPayloadError> {
  Client::default()
    .get(endpoint)
    .header("User-Agent", "Actix-web")
    .send()
    .await
    .unwrap()
    .json()
    .await
}
