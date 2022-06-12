use actix_web::{
    get, post,
    web, http::header,
    HttpResponse, Responder,
    http::header::ContentType,
};
use tera::{Context, Tera};
use lazy_static::lazy_static;
use crate::models;
use reqwest::{self, Url};

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


#[post("/new")]
pub async fn post_new_question(new_question: web::Form<models::NewQuestion>) -> impl Responder {
    println!("{}", new_question.body);
    let destination = "/ui/top";
    HttpResponse::TemporaryRedirect()
        .append_header((header::LOCATION, destination))
        .finish()
}


#[get("/list")]
pub async fn list_questions() -> impl Responder {
    // let _request_path = "http://".to_owned() + header::HOST.as_str() + "/questions/";
    let _request_path = "http://localhost:8080/questions/";
    let request_path = Url::parse(&_request_path).unwrap();
    println!("{}", &request_path);
    let questions = match reqwest::get(request_path.as_str()).await {
        Ok(res) => res,
        Err(e) => {
            println!("{}", &e);
            ::std::process::exit(1);
        }
    }.text()
    .await
    .unwrap();
    println!("{:?}", &questions);

    let mut context = Context::new();
    let html_body = match TEMPLATES.render("list.html", &context) {
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
