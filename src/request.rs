use actix_web::client::{Client, JsonPayloadError};
use actix_web::web;
use serde::{Deserialize, Serialize};

pub async fn request<T>(endpoint: &str, data: web::Json<T>) -> Result<T, JsonPayloadError>
where
  for<'de> T: Serialize + Deserialize<'de>,
{
  Client::default()
    .post(endpoint)
    .send_json(&data.into_inner())
    .await
    .unwrap()
    .json()
    .await
}
