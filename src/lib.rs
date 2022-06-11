#[macro_use]
extern crate diesel;
extern crate tera;
extern crate lazy_static;

pub mod schema;
pub mod db;
pub mod models;
pub mod handler;
pub mod routes;
