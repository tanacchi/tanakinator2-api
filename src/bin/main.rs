use actix_web::{App, HttpServer, middleware::Logger};
use tanakinator2_api::{handler, routes};
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(
        env_logger::Env::new().default_filter_or("info")
    );

    HttpServer::new(|| {
        App::new()
            .service(handler::common::root)
            .configure(routes::app_config)
            .wrap(Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
