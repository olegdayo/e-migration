use diesel::prelude::*;
use std::env;

pub fn create_connection() -> PgConnection {
    let database_url = env::var("KRUSTY_DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(format!("Error connecting to {}", database_url).as_str())
}
