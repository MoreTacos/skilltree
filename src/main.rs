#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate crypto;

mod core;
mod pages;
mod api;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use sled_extensions::DbExt;
use rocket_contrib::templates::tera::Context;
use crate::core::Database;

#[catch(404)]
fn not_found() -> Template {
    let context = Context::new();
    Template::render("error", &context)
}

fn ignite() -> rocket::Rocket {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Database {
            gyms: db
                .open_bincode_tree("gyms")
                .expect("failed to open gym tree"),
        })
        .mount("/static", StaticFiles::from("static"))
        .mount("/", pages::base::routes())
        .mount("/api/gym", api::routes())
        .register(catchers![not_found])
}

fn main() {
    ignite().launch();
}
