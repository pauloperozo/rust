use actix_web::{HttpServer,App,HttpResponse,web,http::header,get};
 
#[get("/")]
async fn helloWord() -> HttpResponse {
    HttpResponse::Ok().content_type(header::ContentType::json()).body("{\"message\":\"Hello, World!\"}")    
}


#[actix_web::main]
async  fn main()-> std::io::Result<()>{

    HttpServer::new(|| {
        App::new().service(helloWord)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await

}