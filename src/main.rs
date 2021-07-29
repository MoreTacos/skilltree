#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate crypto;

mod core;
mod routes;

use crate::core::Database;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::Template;

#[catch(404)]
fn not_found() -> Template {
    let context = Context::new();
    Template::render("error", &context)
}

fn ignite() -> rocket::Rocket {
    let db = sled::open("./database").expect("failed to open database");
    let gyms = db.open_tree("gyms").expect("failed to open gym tree");
    let users = db.open_tree("users").expect("failed to open user tree");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Database::new(gyms, users, "https://skilltreedocs.onrender.com/"))
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes::index())
        .mount("/", routes::sudo())
        .mount("/", routes::admin())
        .mount("/", routes::user())
        .register(catchers![not_found])
}

fn main() {
    ignite().launch();
}
