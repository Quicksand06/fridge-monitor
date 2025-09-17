use axum::{Json, Router, routing::get};
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new().route("/fridge", get(get_items).post(store_item))
}

async fn get_items() -> &'static str {
    "List of items in the fridge"
}

async fn store_item(Json(input): Json<Input>) -> Json<Output> {
    Json(Output {
        echoed: input.barcode,
    })
}

#[derive(Deserialize)]
struct Input {
    barcode: String,
}

#[derive(Serialize)]
struct Output {
    echoed: String,
}
