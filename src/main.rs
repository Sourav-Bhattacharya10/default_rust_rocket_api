use rocket::*;

#[launch] // The "main" function of the program
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
