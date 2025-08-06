/* sys lib */
use axum::Json;
use axum_extra::headers::{authorization::Bearer, Authorization};
use tokio_tungstenite::connect_async;
use url::Url;

/* models */
use crate::models::{
  appstate::AppState,
  response_model::{DataValue, ResponseModel, ResponseStatus},
};

pub struct ConnectionService;

impl ConnectionService {
  pub async fn connect(state: AppState, auth: Authorization<Bearer>) -> Json<ResponseModel> {
    let ws_url = state.ws_address.as_str();

    match Url::parse(ws_url) {
      Ok(_url) => {
        match connect_async(ws_url).await {
          Ok((_ws_stream, _)) => {
            // Connection successful; you can handle the stream here if needed
            // For now, we'll just return a success response
            return Json(ResponseModel {
              status: ResponseStatus::Success,
              message: "Successfully connected to WebSocket server".to_string(),
              data: DataValue::String("".to_string()),
            });
          }
          Err(e) => {
            return Json(ResponseModel {
              status: ResponseStatus::Error,
              message: format!("Failed to connect to WebSocket server: {}", e),
              data: DataValue::String("".to_string()),
            })
          }
        }
      }
      Err(e) => {
        return Json(ResponseModel {
          status: ResponseStatus::Error,
          message: format!("Invalid WebSocket URL: {}", e),
          data: DataValue::String("".to_string()),
        })
      }
    }
    // return Json(ResponseModel {
    //   status: ResponseStatus::Error,
    //   message: "Not Found".to_string(),
    //   data: DataValue::String("".to_string()),
    // });
  }

  pub async fn list_users(auth: Authorization<Bearer>) -> Json<ResponseModel> {
    return Json(ResponseModel {
      status: ResponseStatus::Error,
      message: "Not Found".to_string(),
      data: DataValue::String("".to_string()),
    });
  }
}
