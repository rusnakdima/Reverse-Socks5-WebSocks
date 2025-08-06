/* sys lib */
use axum::{
  routing::{get, post},
  Router,
};

/* controllers */
use crate::{controllers::auth_controller::AuthController, models::appstate};

pub struct AuthRoute;

impl AuthRoute {
  pub async fn root() -> &'static str {
    "Auth Route"
  }

  pub fn create_auth_routes() -> Router<appstate::AppState> {
    Router::new()
      .route("/", get(Self::root))
      .route("/register", post(AuthController::register))
      .route("/login", post(AuthController::login))
      .route("/verify", post(AuthController::verify))
  }
}
