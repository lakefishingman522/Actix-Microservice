use mongodb::{bson::doc, error::Error, options::ClientOptions, Client};

pub async fn db_connect() -> Result<Client, Error> {
  let client_options =
  let client = Client::with_options(client_options)?;
  client
    .database("db_users")
    .run_command(doc! {"ping": 1}, None)
    .await?;
  println!("Connected successfully.");
  // List the names of the databases in that cluster
  for db_name in client.list_database_names(None, None).await? {
    println!("{}", db_name);
  }
  Ok(client)
}
