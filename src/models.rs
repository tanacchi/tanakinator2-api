use crate::schema::questions;
use serde::Serialize;


#[derive(Queryable, Debug, Serialize)]
pub struct Question {
    pub id: i64,
    pub body: Option<String>,
}


#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub body: &'a str,
}
