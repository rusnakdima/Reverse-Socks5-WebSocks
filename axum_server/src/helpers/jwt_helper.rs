/* sys lib */
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};

/* models */
use crate::models::claims::Claims;

pub struct JwtHelper;

impl JwtHelper {
  pub fn new() -> Self {
    Self
  }

  pub async fn parse_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
      token,
      &DecodingKey::from_secret(jwt_secret.as_ref()),
      &Validation::default(),
    )?;
    Ok(token_data.claims)
  }

  pub async fn verify_admin_token(&self, token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
      token,
      &DecodingKey::from_secret(jwt_secret.as_ref()),
      &Validation::default(),
    )?;
    Ok(token_data.claims.role == "admin")
  }

  pub async fn verify_token(&self, token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let token_data = decode::<Claims>(
      token,
      &DecodingKey::from_secret(jwt_secret.as_ref()),
      &Validation::default(),
    )?;
    Ok(token_data.claims.exp > Utc::now().timestamp() as usize)
  }
}
