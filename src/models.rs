use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct User {
  #[serde(default)]
  pub _id: ObjectId,
  #[serde(default)]
  pub username: String,
  #[serde(default)]
  pub email: String,
  #[serde(default)]
  pub password: String,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessToken {
  pub iss: String,
  pub exp: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdToken {
  pub aud: String,
  pub azp: String,
  pub sub: String,
  pub at_hash: String,
  pub email: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct SignInResponse {
  pub access_token: String,
  pub id_token: String,
  pub token_type: String,
  pub expires_in: String,
  pub scope: String,
  #[serde(default)]
  pub refresh_token: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AuthCodeResponse {
  pub access_code: String,
  pub state: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AccessCode {
  pub access_code: String,
  pub expires: String,
}
