/* sys lib */
use axum::Json;
use axum_extra::headers::{authorization::Bearer, Authorization};

/* helpers */
use crate::helpers::db_helper::{authenticate_user, create_user};

use crate::helpers::jwt_helper::JwtHelper;
/* models */
use crate::models::{
  appstate::AppState,
  login_model::LoginReq,
  register_req::RegisterReq,
  response_model::{DataValue, ResponseModel, ResponseStatus},
};

pub struct AuthService;

impl AuthService {
  pub async fn login(state: AppState, req: LoginReq) -> Json<ResponseModel> {
    match authenticate_user(state, &req.username, &req.password).await {
      Ok(Some(token)) => Json(ResponseModel {
        status: ResponseStatus::Success,
        message: "Login successful".to_string(),
        data: DataValue::String(token),
      }),
      Ok(None) => Json(ResponseModel {
        status: ResponseStatus::Error,
        message: "Invalid credentials".to_string(),
        data: DataValue::Object(serde_json::json!({})),
      }),
      Err(e) => Json(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Server error: {}", e),
        data: DataValue::Object(serde_json::json!({})),
      }),
    }
  }

  pub async fn register(
    state: AppState,
    auth: Authorization<Bearer>,
    data: RegisterReq,
  ) -> Json<ResponseModel> {
    let is_admin = JwtHelper::new()
      .verify_admin_token(&state, auth.token())
      .await;
    if !is_admin {
      return Json(ResponseModel {
        status: ResponseStatus::Error,
        message: "Admin access required".to_string(),
        data: DataValue::Object(serde_json::json!({})),
      });
    }
    match create_user(state.db_pool, &data.username, &data.password, &data.role).await {
      Ok(_) => Json(ResponseModel {
        status: ResponseStatus::Success,
        message: "User registered successfully".to_string(),
        data: DataValue::Object(serde_json::json!({})),
      }),
      Err(e) => Json(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("Server error: {}", e),
        data: DataValue::Object(serde_json::json!({})),
      }),
    }
  }

  pub async fn verify(state: AppState, auth: Authorization<Bearer>) -> Json<ResponseModel> {
    let is_valid = JwtHelper::new().verify_token(&state, auth.token()).await;
    if is_valid {
      Json(ResponseModel {
        status: ResponseStatus::Success,
        message: "".to_string(),
        data: DataValue::Object(
          serde_json::value::to_value(JwtHelper::new().parse_token(&state, auth.token()).await)
            .unwrap(),
        ),
      })
    } else {
      Json(ResponseModel {
        status: ResponseStatus::Error,
        message: "".to_string(),
        data: DataValue::String("".to_string()),
      })
    }
  }
}
