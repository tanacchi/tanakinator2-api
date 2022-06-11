use crate::schema::questions;

#[derive(Queryable, Debug)]
pub struct Question {
    pub id: i64,
    pub body: Option<String>,
}


#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub body: &'a str,
}
