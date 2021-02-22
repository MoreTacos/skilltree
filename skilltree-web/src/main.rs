#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::status;
use rocket::Route;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Deserialize;
use serde::Serialize;
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    skills: HashMap<String, usize>,
}

struct Database {
    users: Tree<User>,
}

fn get_users_names(db: &State<Database>) -> Vec<String> {
    db.users
        .iter()
        .map(|user| user.expect("not user").1.username)
        .collect()
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let users = get_users_names(&db);
    #[derive(Serialize)]
    struct Context {
        users: Vec<String>,
    };
    let context = Context { users };
    Template::render("index", &context)
}

#[get("/admin")]
fn admin(db: State<Database>) -> Template {
    let users = get_users_names(&db);
    #[derive(Serialize)]
    struct Context {
        users: Vec<String>,
    }
    let context = Context { users };
    Template::render("admin", &context)
}

#[get("/<username>")]
fn user(db: State<Database>, username: String) -> Template {
    let user = db.users.get(username.as_bytes()).unwrap().unwrap();
    let value = user.skills["front-roll"];

    #[derive(Serialize)]
    struct Context {
        username: String,
        value: usize,
    };
    let context = Context { username, value };

    Template::render("user", &context)
}

#[post("/add-user/<username>")]
fn add_user(db: State<Database>, username: String) -> status::Accepted<String> {
    let username = username.to_string();
    let mut skills = HashMap::new();
    skills.insert("front-roll".to_string(), 0);
    let user = User { username: username.clone(), skills };

    db.users.insert(username.clone().as_bytes(), user).expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[put("/<username>/<skill>/<value>")]
fn update_user(
    db: State<Database>,
    username: String,
    skill: String,
    value: usize,
) -> status::Accepted<String> {
    let mut user = db.users.get(&username.as_bytes()).unwrap().unwrap();
    user.skills.insert(skill, value);
    db.users.insert(username.clone().as_bytes(), user).expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[delete("/remove-user/<username>")]
fn delete_user(db: State<Database>, username: String) -> status::Accepted<String> {
    db.users.remove(&username.as_bytes()).unwrap().unwrap();
    status::Accepted(Some(format!("User {} updated successfully", &username)))
}

fn main() {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Database {
            users: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, user, admin])
        .mount("/api", routes![add_user, update_user, delete_user])
        .launch();
}
