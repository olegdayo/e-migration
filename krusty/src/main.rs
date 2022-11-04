extern crate diesel;
extern crate dotenv;

// pub mod db;
pub mod model;
pub mod schema;

use diesel::prelude::*;
use std::env;
use dotenv::dotenv;

pub fn create_connection() -> PgConnection {
    dotenv()
        .ok()
        .expect("Failed to work with .env");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(format!("Error connecting to {}", database_url).as_str())
}

fn main() {
    let conn = create_connection();
}
