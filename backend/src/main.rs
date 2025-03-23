use rocket::{get, launch, routes};

#[get("/")]
fn index() -> String {
    let version = env!("CARGO_PKG_VERSION");
    format!("Welcome to TierMaster's API\nAPI Version: {version}")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}