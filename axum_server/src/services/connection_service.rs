use std::{
  io::Write,
  net::{Shutdown, TcpStream},
};

/* sys lib */
use axum::Json;
use axum_extra::headers::{authorization::Bearer, Authorization};

/* models */
use crate::models::{
  appstate::AppState,
  claims::Claims,
  response_model::{DataValue, ResponseModel, ResponseStatus},
};

pub struct ConnectionService;

impl ConnectionService {
  pub async fn connect(state: AppState, auth: Authorization<Bearer>) -> Json<ResponseModel> {
    let ws_url = state.ws_address.as_str();
    println!("ws_url: {}", ws_url);

    match TcpStream::connect("127.0.0.1:2020") {
      Ok(mut stream) => {
        let decoded = jsonwebtoken::decode::<Claims>(
          auth.token(),
          &jsonwebtoken::DecodingKey::from_secret(state.jwt_secret.as_ref()),
          &jsonwebtoken::Validation::default(),
        );
        let user_id = match decoded {
          Ok(token_data) => token_data.claims.sub.to_string(),
          Err(e) => {
            return Json(ResponseModel {
              status: ResponseStatus::Error,
              message: format!("Invalid token: {}", e),
              data: DataValue::Object(serde_json::json!({})),
            });
          }
        };

        let command = format!("START {}\n", user_id);
        if let Err(e) = stream.write_all(command.as_bytes()) {
          return Json(ResponseModel {
            status: ResponseStatus::Error,
            message: format!("Failed to send command to server: {}", e),
            data: DataValue::Object(serde_json::json!({})),
          });
        }
        if let Err(e) = stream.flush() {
          return Json(ResponseModel {
            status: ResponseStatus::Error,
            message: format!("Failed to flush command: {}", e),
            data: DataValue::Object(serde_json::json!({})),
          });
        }
        if let Err(e) = stream.shutdown(Shutdown::Both) {
          return Json(ResponseModel {
            status: ResponseStatus::Error,
            message: format!("Failed to shutdown stream: {}", e),
            data: DataValue::Object(serde_json::json!({})),
          });
        }

        Json(ResponseModel {
          status: ResponseStatus::Success,
          message: "Connection signaled to start".to_string(),
          data: DataValue::Object(serde_json::json!({})),
        })
      }
      Err(e) => Json(ResponseModel {
        status: ResponseStatus::Error,
        message: format!("WebSocket server not running: {}", e),
        data: DataValue::Object(serde_json::json!({})),
      }),
    }
  }

  pub async fn list_users(auth: Authorization<Bearer>) -> Json<ResponseModel> {
    return Json(ResponseModel {
      status: ResponseStatus::Error,
      message: "Not Found".to_string(),
      data: DataValue::String("".to_string()),
    });
  }
}
