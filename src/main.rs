use actix_web::{web, App, HttpServer};
use utoipa_swagger_ui::SwaggerUi;
mod api;
mod chat;
use api::ApiDoc;
use utoipa::OpenApi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("🚀 Servidor corriendo en http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(
                web::scope("/api")
                    .service(chat::handlers::chat),
            )
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}