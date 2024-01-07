use rocket::form::validate;
use rocket::serde::json::{json, Json, Value};
use rocket::serde::Deserialize;
use rocket::{get, launch, routes};
use rocket_dyn_templates::{context, Template};

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Option<String>,
    #[validate(length(min = 8))]
    password: Option<String>,
}

#[post("/users", format = "json", data = "<new_user>")]
pub async fn post_users(new_user: Json<NewUser>) -> Result<Value, Errors> {
    let new_user = FieldValidator::validate(&new_user);
    let username = extracter.extract("username", new_user.username);
    let email = extracter.extract("email", new_user.email);
    let password = extracter.extract("username", new_user.password);
}

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
    let rocket = rocket::build()
        .mount("/", routes![index, id])
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
