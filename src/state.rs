use hmac::{Hmac, NewMac};
use mongodb::Client;
use sha2::Sha256;

#[derive()]
pub struct AppState {
  pub app_name: String,
  pub db: mongodb::Client,
  pub private_key: Hmac<Sha256>,
}
