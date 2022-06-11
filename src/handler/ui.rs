use actix_web::{
    get, HttpResponse, Responder,
};


#[get("/top")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
