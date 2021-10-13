use hmac::{Hmac, NewMac};
use reddb::RonDb;
use sha2::Sha256;

#[derive()]
pub struct AppState {
  pub app_name: String,
  pub db: RonDb,
  pub private_key: Hmac<Sha256>,
}
