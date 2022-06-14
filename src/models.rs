use std::convert::From;
use json::JsonValue;
use crate::schema::questions;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Debug, Serialize, PartialEq)]
pub struct Question {
    pub id: i64,
    pub body: Option<String>,
}


impl From<&JsonValue> for Question {
    fn from(json: &JsonValue) -> Self {
        Question {
            id: json["id"].as_i64().unwrap(),  // TODO: Make it general.
            body: json["body"].as_str().map(String::from)
        }
    }
}


#[derive(Debug, Serialize, PartialEq)]
pub struct Questions ( Vec<Question> );


impl Questions {
    pub fn new(questions: Vec<Question>) -> Self {
        Questions(questions)
    }
}


impl From<JsonValue> for Questions {
    fn from(_: JsonValue) -> Self {
        let expected = vec![
            Question{ id: 1, body: Some(String::from("Question 1"))},
            Question{ id: 2, body: Some(String::from("Question 2"))},
            Question{ id: 3, body: Some(String::from("Question 3"))},
        ];
        Questions(expected)
    }
}

#[derive(Insertable, Deserialize)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}
