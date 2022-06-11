use actix_web::{
    web, get, HttpResponse, Responder, Result,
    http::header,
};
use crate::db;


#[get("/")]
pub async fn hello() -> impl Responder {
    let destination = "/ui/top";
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, destination))
        .finish()
}

#[get("/top")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[get("/")]
pub async fn get_all_questions() -> Result<impl Responder> {
    let connection = db::establish_connection();
    let all_questions = db::load_all_questions(&connection);
    Ok(web::Json(all_questions))
}
