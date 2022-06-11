use actix_web::{
    get, HttpResponse, Responder,
    http::header::ContentType,
};
use tera::{Context, Tera};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec!["html", ".sql"]);  // TODO: needs dot or not ?
        tera
    };
}


#[get("/top")]
pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[get("/new")]
pub async fn render_new_question_form() -> impl Responder {
    let mut context = Context::new();
    let html_body = match TEMPLATES.render("add.html", &context) {
        Ok(s) => s,
        Err(e) => {
            println!("Rendering error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    HttpResponse::Ok()
        .insert_header(
            ContentType::html()
        )
        .body(html_body)
}

