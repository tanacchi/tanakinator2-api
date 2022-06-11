use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use tanakinator2_api::db;
use tanakinator2_api::models;
use tanakinator2_api::schema::questions as questions_schema;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = db::establish_connection();
    let new_question = models::NewQuestion {
        body: "Question !!!"
    };
    diesel::insert_into(questions_schema::dsl::questions)
        .values(new_question)
        .execute(&connection)
        .expect("Error saving new question");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
