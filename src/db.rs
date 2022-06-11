use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use dotenv;
use std::env;

pub fn establish_connection() -> MysqlConnection {
    dotenv::from_filename("mysql.env").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}
