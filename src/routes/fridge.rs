use crate::models::{CreateItem, ErrorResponse, ItemResponse};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Json, Router, routing::get};
use sqlx::PgPool;
use uuid::Uuid;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/fridge/items", get(get_items).post(store_item))
        .route("/fridge/items/{id}", get(get_item))
}

async fn get_items(State(pool): State<PgPool>) -> Result<Json<Vec<ItemResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let items = sqlx::query_as::<_, ItemResponse>(r#"SELECT * FROM items"#)
        .fetch_all(&pool)
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Database error: {}", err),
                }),
            )
        })?;

    Ok(Json(items))
}

async fn get_item(Path(id): Path<Uuid>, State(pool): State<PgPool>) -> Result<Json<ItemResponse>, (StatusCode, Json<ErrorResponse>)> {
    let item = sqlx::query_as::<_, ItemResponse>(r#"SELECT * FROM items WHERE id = $1"#)
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: format!("Item not found: {}", err),
                }),
            ),
            other => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: format!("Database error: {}", other),
                }),
            ),
        })?;
    Ok(Json(item))
}

async fn store_item(State(pool): State<PgPool>, Json(input): Json<CreateItem>) -> Result<Json<ItemResponse>, (StatusCode, Json<ErrorResponse>)> {
    let command = sqlx::query_as::<_, ItemResponse>(
        r#"insert into items (name, barcode)
        values ($1, $2)
        returning id, name, barcode
        "#,
    )
    .bind(input.name)
    .bind(input.barcode)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                message: format!("Database error: {}", err),
            }),
        )
    })?;

    Ok(Json(command))
}
