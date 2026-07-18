use actix_web::{HttpServer,App,web};
mod chat;
 
#[actix_web::main]
async  fn main()-> std::io::Result<()>{

    HttpServer::new(|| {
        App::new()
        .service( 
            web::scope("/api")
            .service(chat::handlers::chat)
         )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}