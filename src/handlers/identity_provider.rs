use crate::state::AppState;
use crate::user::User;
use actix_web::{web, HttpResponse, Responder};

pub async fn find_user(data: web::Json<User>, state: web::Data<AppState>) -> impl Responder {
  let db = &state.db;
  let user = db.find(&data.into_inner()).await.unwrap();
  let user_data = user.first().unwrap();
  HttpResponse::Ok().json(user_data)
}
