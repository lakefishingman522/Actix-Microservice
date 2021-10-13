use crate::models::{AccessToken, IdToken, User};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use reddb::Document;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Result;

pub fn get_key() -> Hmac<Sha256> {
  dotenv().ok();
  let private_key = &env::var("PRIVATE_KEY").unwrap();
  let content = fs::read_to_string(private_key).expect("Can't read private key");
  let key: Hmac<Sha256> = Hmac::new_from_slice(content.as_bytes()).unwrap();
  key
}

pub fn generate_token<T>(key: &Hmac<Sha256>, payload: T) -> String
where
  for<'de> T: Serialize + Deserialize<'de>,
{
  let mut token = BTreeMap::new();
  token.insert("payload", payload);
  token.sign_with_key(key).unwrap()
}

pub fn generate_access_token(key: &Hmac<Sha256>) -> String {
  let payload = AccessToken {
    iss: env::var("ISSUER").unwrap(),
    exp: Utc::now().to_string(),
  };
  let mut token = BTreeMap::new();
  token.insert("payload", payload);
  token.sign_with_key(key).unwrap()
}

pub fn generate_id_token(key: &Hmac<Sha256>, doc: Document<User>) -> String {
  let mut claims = BTreeMap::new();
  let payload = IdToken {
    _id: doc._id.to_string(),
    username: doc.data.username,
  };
  claims.insert("payload", payload);
  claims.sign_with_key(key).unwrap()
}

pub fn verify_token(token_str: &str) -> Result<BTreeMap<String, String>> {
  let key = get_key();
  let claims = token_str.verify_with_key(&key).unwrap();
  Ok(claims)
}
