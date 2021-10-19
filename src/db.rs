use mongodb::{bson::doc, error::Error, options::ClientOptions, Client};
use std::env;

use crate::error::CustomError;

pub async fn db_connect() -> Result<Client, CustomError> {
  let client_options = ClientOptions::parse(&env::var("MONGO_DB_URI").unwrap())
    .await
    .unwrap();

  let client = Client::with_options(client_options).unwrap();
  client
    .database("db_users")
    .run_command(doc! {"ping": 1}, None)
    .await
    .unwrap();
  Ok(client)
}
