use actix_web::{
    get, post,
    web, http::header,
    HttpResponse, Responder,
    http::header::ContentType,
};
use tera::{Context, Tera};
use lazy_static::lazy_static;
use crate::models::{self, Question, Questions};
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
    let questions: Questions = json::parse(&questions).unwrap().into();
    let questions: Vec<Question> = questions.into();

    let mut context = Context::new();
    context.insert("questions", &questions);
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


#[cfg(test)]
mod tests {
    use crate::models::{Question, Questions};

    #[test]
    fn should_parse_json_to_questions() {
        use json;

        let response_body = "[{\"id\":1,\"body\":\"Question 1\"},{\"id\":2,\"body\":\"Question 2\"},{\"id\":3,\"body\":\"Question 3\"}]";
        let parsed = json::parse(response_body).unwrap();

        assert_eq!(1, parsed[0]["id"]);
        assert_eq!("Question 1", parsed[0]["body"]);

        let expected  = Question{ id: 1, body: Some(String::from("Question 1"))};
        assert_eq!(expected, Question::from(&parsed[0]));

        let _expected = vec![
            Question{ id: 1, body: Some(String::from("Question 1"))},
            Question{ id: 2, body: Some(String::from("Question 2"))},
            Question{ id: 3, body: Some(String::from("Question 3"))},
        ];
        let expected = Questions::new(_expected);
        assert_eq!(expected, Questions::from(parsed))
    }
}
