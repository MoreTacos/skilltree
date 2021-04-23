use rocket::Route;
use serde::Deserialize;
use crate::core::Database;
use std::collections::HashMap;
use crate::core::Gym;
use crate::core::DatabaseExt;
use crate::core::User;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};
use rocket::request::Form;
use rocket::response::status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest};
use rocket_contrib::json::Json;

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

#[post("/login", data="<login>")]
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
    Template::render("dashboard", &context)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddUser {
    name: String,
    iscoach: bool,
    tabs: String,
}

#[post("/add-user", data="<user>")]
fn add_user(db: State<Database>, gym: Gym, user: Json<AddUser>) -> Json<String> {
    let name: String = user.name.clone();
    let skills: HashMap<String, usize> = HashMap::new();
    let athletes: Vec<String> = vec![];
    let tabs: Vec<String> = {
        if user.tabs == "rec" {
            vec!["fxtree".to_string(), "hbtree".to_string()]
        } else if user.tabs == "adv-rec-boys" {
            vec!["pbtree".to_string(), "phtree".to_string()]
        } else if user.tabs == "adv-rec-girls" {
            vec!["fxtree".to_string(), "vttree".to_string()]
        } else if user.tabs == "mens-comp" {
            vec!["fxtree".to_string(), "phtree".to_string(), "srtree".to_string(), "vttree".to_string(), "pbtree".to_string(), "hbtree".to_string()]
        } else {
            vec!["fxtree".to_string(), "fulltree".to_string()]
        }
    };
    let user: User = User::new(name, skills, athletes, tabs);
    db.add_user(&gym.email, user.clone()).unwrap();
    Json(user.hash)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct RemoveUser {
    hash: String,
}

#[delete("/remove-user", data="<user>")]
fn remove_user(db: State<Database>, gym: Gym, user: Json<RemoveUser>) -> Json<String> {
    db.remove_user(&gym.email, &user.hash).unwrap();
    Json("Removed User".to_string())
}

#[get("/logout")]
fn logout(_gym: Gym, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("email"));
    Redirect::to("/")
}
