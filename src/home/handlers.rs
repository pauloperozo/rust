use actix_web:: {HttpResponse,http::header,get};

#[get("/")]
async fn hello_word() -> HttpResponse {

    HttpResponse::Ok()
    .content_type(header::ContentType::json())
    .body("{\"message\":\"Hello, World!\"}")    

}