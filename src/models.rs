use crate::schema::questions;
use serde::{Serialize, Deserialize};


#[derive(Queryable, Debug, Serialize)]
pub struct Question {
    pub id: i64,
    pub body: Option<String>,
}


#[derive(Insertable, Deserialize)]
#[table_name = "questions"]
pub struct NewQuestion {
    pub body: String,
}
