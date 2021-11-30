use crate::models::token::{AccessToken, IdToken};
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};

use magic_crypt::MagicCryptTrait;

use sha2::Sha256;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Result;

pub fn get_key() -> Hmac<Sha256> {
  let private_key = &env::var("PRIVATE_KEY").unwrap();
  //let content = fs::read_to_string(private_key).expect("Can't read private key");
  let key: Hmac<Sha256> = Hmac::new_from_slice(private_key.as_bytes()).unwrap();
  key
}

pub fn generate_access_code(key: &str) -> String {
  let mc = new_magic_crypt!("private_key", 256);
  let base64 = mc.encrypt_str_to_base64(key);
  base64
}

pub fn generate_access_token(key: &Hmac<Sha256>, token: AccessToken) -> String {
  let mut claims = BTreeMap::new();
  claims.insert("aud", token.iss);
  claims.insert("exp", token.exp);
  claims.insert("sub", token.sub);
  claims.insert("azp", token.azp);
  claims.sign_with_key(key).unwrap()
}

pub fn generate_id_token(key: &Hmac<Sha256>, token: IdToken) -> String {
  let mut claims = BTreeMap::new();
  claims.insert("aud", token.aud);
  claims.insert("azp", token.azp);
  claims.insert("at_hash", token.at_hash);
  claims.insert("sub", token.sub);
  claims.insert("email", token.email);
  claims.sign_with_key(key).unwrap()
}

pub fn decode_token(token_str: &str) -> Result<BTreeMap<String, String>> {
  let key = get_key();
  let claims = token_str.verify_with_key(&key).unwrap();
  Ok(claims)
}
