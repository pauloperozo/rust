use actix_web:: {HttpResponse,get};
use serde::Serialize;


#[derive(Serialize)]
struct MessageResponse {
    message: String 
}


#[get("/")]
async fn hello_word() -> HttpResponse {

    let response = MessageResponse {
        message: "Hello, World!".to_string(),
    };  

    HttpResponse::Ok().json(response)
}