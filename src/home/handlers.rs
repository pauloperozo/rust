use actix_web:: {HttpResponse,get};
use serde::Serialize;


#[derive(Serialize)]
struct MessageResponse {
    message: String 
}


#[get("/say/hello")]
async fn hello_world() -> HttpResponse {

    let response = MessageResponse {
        message: "Hello, World!".to_string(),
    };  

    HttpResponse::Ok().json(response)
}

#[get("/say/goodbye")]
async fn goodbye_world() -> HttpResponse {

    let response = MessageResponse {
        message: "Goodbye, World!".to_string(),
    };  

    HttpResponse::Ok().json(response)
}