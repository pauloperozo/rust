use actix_web:: {post, HttpResponse, web::Json};
use crate::chat::dto::{ChatRequest, ChatResponse}; 

#[post("/chat")]
async fn chat(req: Json<ChatRequest>) -> HttpResponse {

    let enquiry = &req.enquiry; 

    let response = ChatResponse {
        message: format!("You said: {}", enquiry),
    };  

    HttpResponse::Ok().json(response)
}

