/* sys lib */
use axum::{routing::get, Router};

/* controllers */
use crate::{controllers::connection_controller::ConnectionController, models::appstate};

pub struct ConnectionRoute;

impl ConnectionRoute {
  async fn root() -> &'static str {
    "Connection Route"
  }

  pub fn create_connection_route() -> Router<appstate::AppState> {
    Router::new()
      .route("/", get(Self::root))
      .route("/start", get(ConnectionController::connect))
      .route("/list-users", get(ConnectionController::list_users))
  }
}
