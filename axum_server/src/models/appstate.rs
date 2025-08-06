use std::sync::Arc;

use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct AppState {
  pub db_pool: Pool<Postgres>,
  pub ws_address: Arc<String>,
}
