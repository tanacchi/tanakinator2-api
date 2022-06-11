use actix_web::web;
use crate::handler;


pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/ui")
                .service(handler::manual_hello)
        )
        .service(
            web::scope("/question")
                .service(handler::manual_hello)
        );
}
