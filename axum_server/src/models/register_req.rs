/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterReq {
  pub username: String,
  pub password: String,
  pub role: String,
}
