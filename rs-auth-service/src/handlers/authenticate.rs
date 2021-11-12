use actix_http::http::uri;
use actix_web::{web, HttpResponse};
use bson;
use mongodb::bson::doc;

use bson::oid::ObjectId;
use chrono::Utc;
use serde_qs;
use std::env;

use crate::error::CustomError;
use crate::metrics;
use crate::models::{AuthCodeResponse, FormParams, User};
use crate::request::request;
use crate::state::AppState;
use crate::token;

pub async fn authenticate(
  form: web::Form<FormParams>,
  state: web::Data<AppState>,
) -> Result<HttpResponse, CustomError> {
  let metric_families = prometheus::gather();

  let auth_endpoint = env::var("IDENTITY_ENDPOINT").unwrap();

  let data = web::Json(User {
    _id: ObjectId::new(),
    username: form.username.clone(),
    email: "".to_owned(),
    password: form.password.clone(),
  });

  let user = request::<User, User>(&auth_endpoint, data)
    .await
    .map_err(|_e| CustomError::NotFound)?;

  let access_code = token::generate_access_code(
    &[
      form.client_id.clone(),
      form.redirect_url.clone(),
      Utc::now().to_string(),
    ]
    .concat(),
  );

  println!("[Login] Access code: {:?}", access_code.clone());
  let db = &state.db.database("auth-db").collection("tokens");

  let _insert_result = db
    .insert_one(
      doc! {
         "access_code": &access_code,
         "username": &user.username,
         "user_id": &user._id,
         "client_id": &form.client_id,
         "expires": &Utc::now().to_rfc2822() // TODO add LIFE_SPAN
      },
      None,
    )
    .await
    .unwrap();

  let response = AuthCodeResponse {
    access_code: access_code,
    state: form.state.clone(),
  };

  let query = serde_qs::to_string(&response);
  let redirect_url = [form.redirect_url.clone(), query.unwrap()].concat();
  let uri = uri::Builder::new()
    .path_and_query(redirect_url)
    .build()
    .unwrap();

  let response = HttpResponse::Found()
    .header("location", uri.to_string())
    .finish();
  Ok(response)
}
