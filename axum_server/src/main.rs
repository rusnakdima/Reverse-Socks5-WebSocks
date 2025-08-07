/* imports */
mod controllers;
mod helpers;
mod models;
mod routes;
mod services;

/* sys lib */
use std::env;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

/* helpers */
use crate::helpers::db_helper::DbHelper;

/* models */
use crate::models::appstate;

/* routes */
use crate::routes::main_route::MainRoute;

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let address = env::var("ADDRESS").unwrap_or_else(|_| "0.0.0.0:7878".to_string());
  let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

  let state = appstate::AppState {
    db_pool: DbHelper::new().await.pool,
    ws_address: Arc::new("wss://127.0.0.1:2020".to_string()),
    jwt_secret: jwt_secret,
  };

  // Create Axum router
  let app = MainRoute::create_router()
    .await
    .layer(CorsLayer::permissive())
    .with_state(state);

  // Start Axum server
  let listener = tokio::net::TcpListener::bind(&address.clone())
    .await
    .expect("Failed to bind");

  println!("Server running on http://{address}");

  axum::serve(listener, app)
    .await
    .expect("Failed to start server");
}
