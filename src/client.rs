use actix_web::client::{Client, ClientResponse, JsonPayloadError, PayloadError};
use serde::{Deserialize, Serialize};

pub async fn request<T>(endpoint: &str) -> Result<T, JsonPayloadError>
where
  for<'de> T: Serialize + Deserialize<'de>,
{
  Client::default()
    .get(endpoint)
    .header("User-Agent", "Actix-web")
    .send()
    .await
    .unwrap()
    .json()
    .await
}
