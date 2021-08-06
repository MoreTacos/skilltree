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
use crate::core::User;
use serde::Serialize;

pub fn admin() -> Vec<Route> {
    routes![base, login, dashboard, logout, add_user, add_group, remove_user]
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

#[derive(Serialize)]
struct GroupContext {
    name: String,
    size: usize,
    groupurl: String,
    users: Vec<User>
}

#[get("/dashboard")]
fn dashboard(db: State<Database>, gym: Gym) -> Template {
    let users = db.get_gym_users(&gym.email).unwrap();
    let groups = db.get_gym_groups(&gym.email).unwrap();
    let mut groups: Vec<GroupContext> = groups.into_iter().map(|g| {
        let users = db.get_group_users(&g.groupurl).unwrap();
        let size = users.len();
        let group = GroupContext {
            name: g.name,
            size,
            groupurl: g.groupurl,
            users,
        };
        group
    })
    .collect();
    groups.sort_by(|a, b| a.name.cmp(&b.name));
    let mut context = Context::new();
    context.insert("isadmin", &true);
    context.insert("name", &gym.name);
    context.insert("users", &users);
    context.insert("groups", &groups);
    Template::render("dashboard", &context)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddUser {
    name: String,
    groupurl: String,
}

#[post("/add-user", data = "<user>")]
fn add_user(mut db: State<Database>, gym: Gym, user: Json<AddUser>) -> Json<String> {
    let packageurl = "MAG";
    let skills: HashMap<String, usize> = HashMap::new();
    let user = db.add_user(&user.name, &gym.email, &user.groupurl, &packageurl, skills).unwrap();
    Json(user.userurl)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct AddGroup {
    name: String,
}

#[post("/add-group", data = "<group>")]
fn add_group(mut db: State<Database>, gym: Gym, group: Json<AddGroup>) -> Json<String> {
    dbg!(&group.name);
    let group = db.add_group(&group.name, &gym.email).unwrap();
    Json(group.groupurl)
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
struct RemoveUser {
    userurl: String,
}

#[delete("/remove-user", data = "<user>")]
fn remove_user(mut db: State<Database>, gym: Gym, user: Json<RemoveUser>) -> Json<String> {
    let user = db.get_user(&user.userurl).unwrap().unwrap();
    assert_eq!(gym.email, user.gymemail);
    db.remove_user(&user.userurl).unwrap();
    Json("Removed User".to_string())
}

#[get("/logout")]
fn logout(_gym: Gym, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("email"));
    Redirect::to("/")
}
