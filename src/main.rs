use actix_web::{HttpServer,App};
mod home;
use home::handlers::{hello_world, goodbye_world};
 
#[actix_web::main]
async  fn main()-> std::io::Result<()>{

    HttpServer::new(|| {
        App::new()
        .service(hello_world)
        .service(goodbye_world)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}