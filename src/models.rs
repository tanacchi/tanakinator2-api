use crate::schema::questions;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: i64,
    pub body: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Questions ( Vec<Question> );


impl Into<Vec<Question>> for Questions {
    fn into(self) -> Vec<Question> {
        self.0
    }
}


#[derive(Insertable, Deserialize)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}
