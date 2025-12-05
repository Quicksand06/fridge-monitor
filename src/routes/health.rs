use axum::{Router, routing::get};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route("/health", get(health_check))
}

async fn health_check() -> &'static str {
    "Healthy"
}
