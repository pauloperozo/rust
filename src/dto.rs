use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WebhookRequest {
    #[schema(example = "John Doe")]
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct WebhookResponse {
    pub message: String,
}
