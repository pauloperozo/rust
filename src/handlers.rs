use std::println;

use crate::dto::{WebhookRequest, WebhookResponse};
use actix_web::{post, web::Json};

#[utoipa::path(
    post,
    tag = "Webhook",
    path = "/webhook",
    request_body = WebhookRequest,
    responses(
        (status = 201, description = "Respuesta del Webhook", body = WebhookResponse)
    )
)]
#[post("/webhook")]
pub async fn webhook(req: Json<WebhookRequest>) -> Json<WebhookResponse> {
    let name = &req.name;
    let response = WebhookResponse {
        message: "Message received".to_string(),
    };
    println!("Input {}", name);
    Json(response)
}
