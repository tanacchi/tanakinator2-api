use actix_web::{get, HttpResponse, Responder};


#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("hello")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

