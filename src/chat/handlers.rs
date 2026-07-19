use crate::chat::dto::{ChatRequest, ChatResponse};
use actix_web::{post, web::Json};

#[utoipa::path(
    post,
    tag = "Chat",
    path = "/api/chat",
    request_body = ChatRequest,
    responses(
        (status = 200, description = "Respuesta del chat", body = ChatResponse)
    )
)]
#[post("/chat")]
pub async fn chat(req: Json<ChatRequest>) -> Json<ChatResponse> {
    let enquiry = &req.enquiry;

    let response = ChatResponse {
        message: format!("You said: {}", enquiry),
    };

    Json(response)
}
