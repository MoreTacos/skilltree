use crate::core::Database;
use crate::core::DatabaseExt;
use crate::core::Gym;
use rocket::http::{Cookie, Cookies};
use rocket::request::Form;
use rocket::response::status;
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::tera::Context;
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
    if db
        .verify_gym(&login.email, &login.pw)
        .unwrap()
        .unwrap_or(false)
    {
        cookies.add_private(Cookie::new("email", login.email.to_string()));
        Redirect::to("/dashboard")
    } else {
        Redirect::to("/")
    }
}

#[get("/dashboard")]
fn dashboard(db: State<Database>, gym: Gym) -> Template {
    let mut users = db.get_gym_users(&gym.email).unwrap();
    users.sort_by(|a, b| a.name.cmp(&b.name));
    let mut context = Context::new();
    context.insert("isadmin", &true);
    context.insert("name", &gym.name);
    context.insert("users", &users);
    Template::render("dashboard", &context)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddUser {
    name: String,
}

#[post("/add-user", data = "<user>")]
fn add_user(mut db: State<Database>, gym: Gym, user: Json<AddUser>) -> Json<String> {
    let packageurl = "MAG";
    let skills: HashMap<String, usize> = HashMap::new();
    let user = db.add_user(&user.name, &gym.email, &packageurl, skills).unwrap();
    Json(user.userurl)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct RemoveUser {
    hash: String,
}

#[delete("/remove-user", data = "<user>")]
fn remove_user(mut db: State<Database>, gym: Gym, user: Json<RemoveUser>) -> Json<String> {
    let user = db.get_user(&user.hash).unwrap().unwrap();
    assert_eq!(gym.email, user.gymemail);
    db.remove_user(&user.userurl).unwrap();
    Json("Removed User".to_string())
}

#[get("/logout")]
fn logout(_gym: Gym, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("email"));
    Redirect::to("/")
}
