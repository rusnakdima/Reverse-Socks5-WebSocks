/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginReq {
  pub username: String,
  pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
  pub token: String,
}
