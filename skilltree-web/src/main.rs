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

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    name: String,
    skills: Vec<usize>,
}

struct Database {
    users: Tree<User>,
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    match db.users.get("admin2").expect("DB is up") {
        Some(v) => (),
        None => {
            let admin = User {
                name: "admin2".to_string(),
                skills: vec![0],
            };
            println!("inserting admin2");
            db.users.insert("admin2", admin).expect("Inserting admin");
            ()
        }
    }
    let len = db.users.len();
    let users: Vec<String> = db
        .users
        .iter()
        .take(len - 1)
        .map(|user| user.expect("not user").1.name)
        .collect();
    #[derive(Serialize)]
    struct Context {
        users: Vec<String>,
    };

    let context = Context { users };
    Template::render("index", &context)
}

#[get("/admin")]
fn admin(db: State<Database>) -> Template {
    todo!()
}

#[get("/<name>")]
fn user(db: State<Database>, name: String) -> Template {
    todo!()
    /*
    match db.users.get("front-roll").unwrap() {
        Some(v) => (),
        None => {
            db.values.insert("front-roll", 0 as usize).unwrap();
            ()
        }
    }
    let value = db.values.get("front-roll").unwrap().unwrap();
    #[derive(Serialize)]
    struct Context {
        title: String,
        value: usize,
    };

    let context = Context {
        title: "T".to_string(),
        value,
    };

    Template::render("index", &context)
        */
}

#[post("/front-roll/<value>")]
fn update(db: State<Database>, value: usize) -> status::Accepted<String> {
    todo!()
    /*
    println!("{} saved to the database", &value);
    db.users.insert("front-roll", value).unwrap();
    status::Accepted(Some(format!("value: {}", &value)))
        */
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
        .mount("/", routes![index, user])
        .mount("/api", routes![update])
        .launch();
}
