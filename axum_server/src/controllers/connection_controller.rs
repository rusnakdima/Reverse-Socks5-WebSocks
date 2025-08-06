use axum::{extract::State, Json};
/* sys lib */
use axum_extra::{
  headers::{authorization::Bearer, Authorization},
  TypedHeader,
};

/* models */
use crate::models::{appstate::AppState, response_model::ResponseModel};

/* services */
use crate::services::connection_service::ConnectionService;

pub struct ConnectionController;

impl ConnectionController {
  pub async fn connect(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
  ) -> Json<ResponseModel> {
    ConnectionService::connect(state, auth).await
  }

  pub async fn list_users(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
  ) -> Json<ResponseModel> {
    ConnectionService::list_users(auth).await
  }
}
