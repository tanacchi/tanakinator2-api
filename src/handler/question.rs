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
