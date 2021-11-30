use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
  pub response_type: String,
  pub client_id: String,
  pub redirect_url: String,
  pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormParams {
  pub username: String,
  pub password: String,
  pub redirect_url: String,
  pub client_id: String,
  #[serde(default)]
  pub state: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthCodeResponse {
  pub access_code: String,
  pub state: String,
}
