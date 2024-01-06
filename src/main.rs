#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/id")]
fn id() -> &'static str {
    "id is 1"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![id])
}
