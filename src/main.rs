extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    sports_analytics::rocket().launch().await?;
    Ok(())
}
