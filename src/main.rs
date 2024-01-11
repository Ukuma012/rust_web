use rocket::serde::Deserialize;
use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing());

    let figment = rocket.figment();
    #[derive(Deserialize, Debug)]
    #[serde(crate = "rocket::serde")]
    struct Config {
        port: u16,
    }

    let config: Config = figment.extract().expect("config");
    println!("Server started on port {}", config.port);

    rocket
}
