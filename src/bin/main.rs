use actix_web::{App, HttpServer};
use tanakinator2_api::{handler, routes};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handler::hello)
            .configure(routes::app_config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
