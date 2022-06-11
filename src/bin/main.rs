use actix_web::{App, HttpServer};
use tanakinator2_api::{db, handler, routes};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = db::establish_connection();
    db::insert_new_question(&connection, String::from("Ahi"));
    let all_questions = db::load_all_questions(&connection);
    all_questions.iter().for_each(|q|
        println!("{:?}", q)
    );

    HttpServer::new(|| {
        App::new()
            .service(handler::hello)
            .configure(routes::app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
