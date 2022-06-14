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


#[derive(Insertable, Deserialize)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}
