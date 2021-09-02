use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
  pub uuid: String,
  pub email: String,
  pub password: String,
}
