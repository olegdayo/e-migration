extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate router;

mod model;
mod schema;
mod db;
mod tasks;
mod server;

use dotenv::dotenv;
use tasks::TaskRunner;
use server::Server;

fn main() {
    dotenv().ok().expect("Failed to work with .env");
    let server = Server::new(
        TaskRunner::new()
            .get_answers()
    );
    server.start().unwrap();
}
