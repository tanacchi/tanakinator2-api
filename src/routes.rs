use actix_web::web;
use crate::handler;


pub fn app_config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/ui")
                .service(handler::ui::manual_hello)
        )
        .service(
            web::scope("/questions")
                .service(handler::question::get_all_questions)
        );
}
