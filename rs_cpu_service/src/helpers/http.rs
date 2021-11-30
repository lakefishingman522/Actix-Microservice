use actix_web::client::{Client, JsonPayloadError};
use actix_web::web;
use serde::{Deserialize, Serialize};

pub async fn request<T, R>(endpoint: &str, data: web::Json<T>) -> Result<R, JsonPayloadError>
where
  for<'de> T: Serialize + Deserialize<'de>,
  for<'de> R: Serialize + Deserialize<'de>,
{
  println!("[Request] to: {:?}", endpoint);

  Client::default()
    .post(endpoint)
    .send_json(&data.into_inner())
    .await
    .unwrap()
    .json()
    .await
}
