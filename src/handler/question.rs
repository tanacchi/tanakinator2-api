use actix_web::{
    web, get, Responder, Result,
};
use crate::db;


#[get("/")]
pub async fn get_all_questions() -> Result<impl Responder> {
    let connection = db::establish_connection();
    let all_questions = db::load_all_questions(&connection);
    Ok(web::Json(all_questions))
}


#[get("/{id}")]
pub async fn get_question_by_id(path: web::Path<(i64,)>) -> Result<impl Responder> {
    let connection = db::establish_connection();
    let question = db::get_question(&connection, path.into_inner().0);
    Ok(web::Json(question))
}
