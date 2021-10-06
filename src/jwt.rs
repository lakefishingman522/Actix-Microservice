use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::user::User;
use dotenv::dotenv;
use std::env;

fn get_key() -> Hmac<Sha256> {
  dotenv().ok();
  let secret = &env::var("SECRET").unwrap();
  let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
  key
}

pub fn generate_token(user: User) -> String {
  let key = get_key();
  let mut claims = BTreeMap::new();
  claims.insert("email", user.email);
  claims.sign_with_key(&key).unwrap()
}

pub fn verify_token(token_str: &str) -> BTreeMap<String, String> {
  let key = get_key();
  let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();
  claims
}
