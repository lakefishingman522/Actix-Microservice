use reddb::RonDb;

#[derive()]
pub struct AppState {
  pub app_name: String,
  pub db: RonDb,
}
