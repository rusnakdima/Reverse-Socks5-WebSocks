/* sys lib */
use axum::{extract::State, Json};
use axum_extra::{
  headers::{authorization::Bearer, Authorization},
  TypedHeader,
};

/* models */
use crate::models::{
  appstate::AppState, login_model::LoginReq, register_req::RegisterReq,
  response_model::ResponseModel,
};

/* services */
use crate::services::auth_service::AuthService;

pub struct AuthController;

impl AuthController {
  pub async fn login(
    State(state): State<AppState>,
    Json(data): Json<LoginReq>,
  ) -> Json<ResponseModel> {
    AuthService::login(state, data).await
  }

  pub async fn register(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<RegisterReq>,
  ) -> Json<ResponseModel> {
    AuthService::register(state, auth, data).await
  }

  pub async fn verify(
    State(state): State<AppState>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
  ) -> Json<ResponseModel> {
    AuthService::verify(state, auth).await
  }
}
