/* sys lib */
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::{postgres::PgPoolOptions, Error, Row};
use std::env;
use uuid::Uuid;

/* models */
use crate::models::{appstate::AppState, claims::Claims};

pub struct DbHelper {
  pub pool: sqlx::Pool<sqlx::Postgres>,
}

impl DbHelper {
  pub async fn new() -> Self {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
      .max_connections(5)
      .connect(&db_url)
      .await
      .expect("Failed to connect to database");

    // Initialize database schema
    // sqlx::migrate!()
    //   .run(&db_pool)
    //   .await
    //   .expect("Failed to run migrations");

    Self { pool: db_pool }
  }
}

pub async fn authenticate_user(
  state: AppState,
  username: &str,
  password: &str,
) -> Result<Option<String>, Error> {
  let user = sqlx::query(
    r#"
        SELECT * FROM users WHERE username = $1
        "#,
  )
  .bind(username)
  .fetch_optional(&state.db_pool)
  .await?;

  if let Some(row) = user {
    let stored_password: String = row.get("password");
    if verify(password, &stored_password).unwrap() {
      let claims = Claims {
        sub: row.get("id"),
        username: row.get("username"),
        role: row.get("role"),
        exp: (Utc::now() + Duration::hours(24)).timestamp() as usize,
      };
      let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
      )
      .unwrap();
      return Ok(Some(token));
    }
  }
  Ok(None)
}

pub async fn create_user(
  pool: sqlx::Pool<sqlx::Postgres>,
  username: &str,
  password: &str,
  role: &str,
) -> Result<(), Error> {
  let hashed_password = hash(password, DEFAULT_COST).unwrap();
  sqlx::query(
    r#"
        INSERT INTO users (id, username, password, role)
        VALUES ($1, $2, $3, $4)
        "#,
  )
  .bind(Uuid::new_v4())
  .bind(username)
  .bind(hashed_password)
  .bind(role)
  .execute(&pool)
  .await?;
  Ok(())
}
