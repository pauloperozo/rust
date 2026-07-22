use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Webhook", description = "API para el webhook")
    ),
    paths(
        crate::handlers::webhook
    ),
)]
pub struct ApiDoc;
