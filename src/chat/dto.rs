use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChatRequest {
    pub enquiry: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChatResponse {
    pub message: String,
}