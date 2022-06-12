use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv;
use std::env;
use crate::models;
use crate::schema::questions as questions_schema;

pub fn establish_connection() -> MysqlConnection {
    dotenv::from_filename("mysql.env").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}


pub fn insert_new_question(connection: &diesel::MysqlConnection, body_str: String) {
    let new_question = models::NewQuestion {
        body: body_str,
    };
    diesel::insert_into(questions_schema::dsl::questions)
        .values(new_question)
        .execute(connection)
        .expect("Error saving new question");
}


pub fn load_all_questions(connection: &diesel::MysqlConnection) -> Vec<models::Question> {
    questions_schema::dsl::questions
        .load::<models::Question>(connection)
        .expect("Error loading questions")
}


pub fn get_question(connection: &diesel::MysqlConnection, id: i64) -> models::Question {
    questions_schema::dsl::questions
        .find(id)
        .get_result::<models::Question>(connection)
        .expect("Error getting the question")
}
