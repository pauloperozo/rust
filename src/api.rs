use utoipa::OpenApi;
use crate::chat::handlers::chat;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Chat", description = "API para el chat")       
    ),
    paths(
        crate::chat::handlers::chat
    ),
)]
pub struct ApiDoc;