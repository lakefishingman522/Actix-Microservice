use actix_web::{web, HttpResponse};

use crate::error::CustomError;
use crate::models::User;
use crate::state::AppState;

pub async fn find_user(
  data: web::Json<User>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let db = &state.db;
  let user = db
    .find(&data.into_inner())
    .await
    .map_err(|_e| CustomError::NotFound)?;
  let user_data = user.first().ok_or(CustomError::NotFound)?;
  Ok(HttpResponse::Ok().json(user_data))
}
