use hmac::Hmac;
use mongodb::Client;
use sha2::Sha256;

#[derive()]
pub struct AppState {
  pub app_name: String,
  pub db: Client,
  pub private_key: Hmac<Sha256>,
}
