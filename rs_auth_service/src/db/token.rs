use actix_web::web;
use mongodb::bson::Document;
use mongodb::error::Error;
use mongodb::results::InsertOneResult;

use crate::models::app_state::AppState;
use crate::models::token::Token;

pub async fn insert(doc: Token, state: &web::Data<AppState>) -> Result<InsertOneResult, Error> {
  let db = &state.db.database("auth-db").collection::<Token>("tokens");
  db.insert_one(doc, None).await
}

pub async fn find_access_code(
  doc: Document,
  state: &web::Data<AppState>,
) -> Result<Option<Token>, Error> {
  let db = &state.db.database("auth-db").collection::<Token>("tokens");
  db.find_one(doc, None).await
}
