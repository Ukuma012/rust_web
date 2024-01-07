use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/id/<id>")]
fn id(id: usize) -> Template {
    Template::render(
        "index",
        context! {
            id: id,
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, id])
        .attach(Template::fairing())
}
