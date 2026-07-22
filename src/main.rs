mod api;
mod dto;
mod handlers;

use actix_web::{App, HttpServer};
use api::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Servidor corriendo en http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new().service(handlers::webhook).service(
            SwaggerUi::new("/docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
