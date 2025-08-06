use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
  pub sub: Uuid,
  pub username: String,
  pub role: String,
  pub exp: usize,
}