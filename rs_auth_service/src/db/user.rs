use actix_web::web;
use mongodb::bson::Document;
use mongodb::error::Error;

use crate::models::app_state::AppState;
use crate::models::user::User;

pub async fn find_one(doc: Document, state: &web::Data<AppState>) -> Result<Option<User>, Error> {
  let db = &state.db.database("auth-db").collection::<User>("users");
  db.find_one(doc, None).await
}
