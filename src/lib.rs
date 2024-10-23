#[macro_use]
extern crate rocket;
extern crate dotenv;

mod ai;
mod database;
mod errors;
mod models;
mod routes;
mod types;

use dotenv::dotenv;
use errors::not_found;
use routes::scores::{add_scores, fetch_all_scores};

// Create test route to ensure API is functional
#[get("/test")]
pub fn test() -> &'static str {
    "Testing"
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .mount("/api", routes![test, fetch_all_scores, add_scores])
        .register("/", catchers![not_found])
}
