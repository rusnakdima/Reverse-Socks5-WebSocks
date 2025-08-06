/* sys lib */
use chrono::Utc;
use jsonwebtoken::{decode, DecodingKey, Validation};

/* models */
use crate::models::{appstate::AppState, claims::Claims};

#[derive(Debug, Clone)]
pub struct JwtHelper {
  pub claims: Claims,
}

impl JwtHelper {
  pub fn new() -> Self {
    Self {
      claims: Claims::default(),
    }
  }

  pub fn decode_token(&self, state: &AppState, token: &str) -> Self {
    let claims = decode::<Claims>(
      token,
      &DecodingKey::from_secret(state.jwt_secret.as_ref()),
      &Validation::default(),
    )
    .unwrap()
    .claims;

    Self { claims: claims }
  }

  pub async fn parse_token(&self, state: &AppState, token: &str) -> Claims {
    self.decode_token(state, token).claims
  }

  pub async fn verify_admin_token(&self, state: &AppState, token: &str) -> bool {
    self.decode_token(state, token).claims.role == "admin"
  }

  pub async fn verify_token(&self, state: &AppState, token: &str) -> bool {
    self.decode_token(state, token).claims.exp > Utc::now().timestamp() as usize
  }
}
