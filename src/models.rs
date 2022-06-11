use crate::schema::questions;

#[derive(Queryable)]
pub struct Question {
    pub id: i64,
    pub body: String,
}


#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub body: &'a str,
}
