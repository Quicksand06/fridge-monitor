use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub barcode: String,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i32,
    pub name: String,
    pub barcode: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
