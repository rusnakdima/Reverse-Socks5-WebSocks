/* sys lib */
use axum::{
  http::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
  },
  routing::get,
  Router,
};
use http::HeaderValue;
use tower_http::cors::CorsLayer;

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

    let router: Router<appstate::AppState> = Router::new()
      .route("/", get(Self::root))
      .nest("/auth", AuthRoute::create_auth_routes())
      .nest("/connection", ConnectionRoute::create_connection_route())
      .layer(cors.clone());

    router
  }
}
