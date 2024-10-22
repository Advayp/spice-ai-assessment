#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;

// Create test route to ensure API is functional
#[get("/test")]
pub fn test() -> &'static str {
    "Testing"
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();

    rocket::build().mount("/api", routes![test])
}
