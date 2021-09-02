use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;
use std::collections::BTreeMap;

use crate::user::User;
use dotenv::dotenv;
use std::env;

pub fn generate_token(user: User) -> String {
  dotenv().ok();
  let secret = &env::var("SECRET").unwrap();
  let key: Hmac<Sha256> = Hmac::new_from_slice(secret.as_bytes()).unwrap();
  let mut claims = BTreeMap::new();
  claims.insert("uuid", user.uuid);
  claims.insert("email", user.email);
  claims.sign_with_key(&key).unwrap()
}
