#[macro_use]
extern crate rocket;
extern crate dotenv;

mod database;
mod models;
mod routes;
mod types;

use dotenv::dotenv;
use routes::scores::{add_scores, fetch_all_scores};

// Create test route to ensure API is functional
#[get("/test")]
pub fn test() -> &'static str {
    "Testing"
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::build().mount("/api", routes![test, fetch_all_scores, add_scores])
}
