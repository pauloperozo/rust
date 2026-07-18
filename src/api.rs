use utoipa::OpenApi;

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