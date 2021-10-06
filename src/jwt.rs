use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use reddb::Document;
use sha2::Sha256;
use std::collections::BTreeMap;
use std::io::Result;

use crate::user::User;
use dotenv::dotenv;
use std::env;

fn get_key() -> Hmac<Sha256> {
  dotenv().ok();
  let secret = &env::var("SECRET").unwrap();
  let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
  key
}

pub fn generate_token(doc: Document<User>) -> String {
  let key = get_key();
  let mut claims = BTreeMap::new();
  claims.insert("email", doc.data.email);
  claims.insert("_id", doc._id.to_string());
  claims.sign_with_key(&key).unwrap()
}

pub fn verify_token(token_str: &str) -> Result<BTreeMap<String, String>> {
  let key = get_key();
  let claims = token_str.verify_with_key(&key).unwrap();
  Ok(claims)
}
