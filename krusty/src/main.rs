extern crate diesel;
extern crate dotenv;

pub mod model;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use model::*;
use std::env;

pub fn create_connection() -> PgConnection {
    dotenv().ok().expect("Failed to work with .env");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(format!("Error connecting to {}", database_url).as_str())
}

fn main() {
    use schema::olympics::countries::dsl::*;
    let mut conn = create_connection();

    let cntrs: Vec<Country> = countries
        .load(&mut conn)
        .expect("Something went wrong with the countries!");

    println!("{}", cntrs.len());
    for cntr in cntrs {
        println!("{:?}", cntr);
    }
}
