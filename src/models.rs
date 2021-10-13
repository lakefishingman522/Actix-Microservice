use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
  #[serde(default)]
  pub _id: String,
  #[serde(default)]
  pub username: String,
  #[serde(default)]
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Params {
  pub response_type: String,
  pub client_id: String,
  pub redirect_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormParams {
  pub username: String,
  pub password: String,
  pub redirect_url: String,
  pub client_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
  pub iss: String,
  pub exp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
  pub _id: String,
  pub username: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignInResponse {
  pub access_token: String,
  pub id_token: String,
  pub token_type: String,
  pub expires_in: String,
}
