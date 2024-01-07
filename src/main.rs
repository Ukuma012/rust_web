#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/<id>")]
fn id(id: usize) -> String {
    format!("id: {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, id])
}
