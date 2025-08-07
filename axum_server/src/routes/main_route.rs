/* sys lib */
use axum::{
  http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
  },
  routing::{get, get_service},
  Router,
};
use http::HeaderValue;
use tower_http::{
  cors::CorsLayer,
  services::{ServeDir, ServeFile},
};

/* routes */
use crate::{
  models::appstate,
  routes::{auth_route::AuthRoute, connection_route::ConnectionRoute},
};

pub struct MainRoute;

impl MainRoute {
  async fn root() -> &'static str {
    "Hello, World!"
  }

  pub async fn create_router() -> Router<appstate::AppState> {
    let cors = CorsLayer::new()
      .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
      .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
      .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let static_files = ServeDir::new("../frontend/dist/frontend/browser").fallback(ServeFile::new(
      "../frontend/dist/frontend/browser/index.html",
    ));

    let router: Router<appstate::AppState> = Router::new()
      .route("/api", get(Self::root))
      .nest("/api/auth", AuthRoute::create_auth_routes())
      .nest(
        "/api/connection",
        ConnectionRoute::create_connection_route(),
      )
      .route_service("/", get_service(static_files.clone()))
      .route_service("/{*path}", get_service(static_files.clone()))
      .layer(cors.clone());

    router
  }
}
