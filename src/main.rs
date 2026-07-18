use actix_web::{HttpServer,App};
mod chat;
 
#[actix_web::main]
async  fn main()-> std::io::Result<()>{

    HttpServer::new(|| {
        App::new()
        .service( chat::handlers::chat )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}