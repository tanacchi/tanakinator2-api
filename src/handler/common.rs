use actix_web::{
    get, HttpResponse, Responder,
    http::header,
};


#[get("/")]
pub async fn root() -> impl Responder {
    let destination = "/ui/top";
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, destination))
        .finish()
}
