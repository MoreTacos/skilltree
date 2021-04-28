use crate::core::Database;
use crate::core::DatabaseExt;
use crate::core::Gym;
use crate::core::User;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::Form;
use rocket::request::{self, FromRequest, Request};
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use serde::Deserialize;
use std::collections::HashMap;

pub fn admin() -> Vec<Route> {
    routes![base, login, dashboard, logout, add_user, remove_user]
}

#[derive(FromForm)]
struct Login {
    email: String,
    pw: String,
}

#[get("/")]
fn base(gym: Gym) -> Template {
    let mut context = Context::new();
    context.insert("isadmin", &true);
    Template::render("index", &context)
}

#[post("/login", data = "<login>")]
fn login(db: State<Database>, mut cookies: Cookies, login: Form<Login>) -> Redirect {
    if db.verify_gym(&login.email, &login.pw) {
        cookies.add_private(Cookie::new("email", login.email.to_string()));
        Redirect::to("/dashboard")
    } else {
        Redirect::to("/")
    }
}

#[get("/dashboard")]
fn dashboard(db: State<Database>, gym: Gym) -> Template {
    let mut context = Context::new();
    context.insert("isadmin", &true);
    context.insert("name", &gym.name);
    context.insert("gymurl", &gym.url);
    context.insert("users", &gym.users);
    context.insert("packages", &gym.packages);
    Template::render("dashboard", &context)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddUser {
    name: String,
    iscoach: bool,
    tabs: String,
}

#[post("/add-user", data = "<user>")]
fn add_user(db: State<Database>, gym: Gym, user: Json<AddUser>) -> Json<String> {
    let name: String = user.name.clone();
    let skills: HashMap<String, usize> = HashMap::new();
    let athletes: Vec<String> = vec![];
    let user: User = User::new(name, skills, athletes, user.tabs.clone());
    db.add_user(&gym.email, user.clone()).unwrap();
    Json(user.hash)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct RemoveUser {
    hash: String,
}

#[delete("/remove-user", data = "<user>")]
fn remove_user(db: State<Database>, gym: Gym, user: Json<RemoveUser>) -> Json<String> {
    db.remove_user(&gym.email, &user.hash).unwrap();
    Json("Removed User".to_string())
}

#[get("/logout")]
fn logout(_gym: Gym, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("email"));
    Redirect::to("/")
}
