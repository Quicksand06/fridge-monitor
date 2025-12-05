use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub barcode: String,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: Uuid,
    pub name: String,
    pub barcode: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
