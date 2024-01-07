use rocket::serde::json::{json, Json, Value};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    user: NewUserData,
}

#[derive(Deserialize, Validate)]
struct NewUserData {
    #[validate(length(min = 1))]
    username: Option<String>,
    #[validate(email)]
    email: Options<String>,
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
