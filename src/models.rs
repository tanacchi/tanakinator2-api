#[derive(Queryable)]
pub struct Question {
    pub id: i64,
    pub body: String,
}
