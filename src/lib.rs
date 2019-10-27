#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}