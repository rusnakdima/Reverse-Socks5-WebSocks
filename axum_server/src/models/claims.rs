use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Claims {
  pub sub: Uuid,
  pub role: String,
  pub exp: usize,
}