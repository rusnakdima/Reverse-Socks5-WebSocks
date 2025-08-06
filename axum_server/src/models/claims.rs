use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
  pub sub: Uuid,
  pub username: String,
  pub role: String,
  pub exp: usize,
}

impl Claims {
  pub fn default() -> Self {
    Self {
      sub: Uuid::new_v4(),
      username: "".to_string(),
      role: "".to_string(),
      exp: 0,
    }
  }
}
