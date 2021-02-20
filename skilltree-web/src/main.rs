#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod pages;

use rocket_contrib::templates::Template;
use serde::Serialize;

#[catch(404)]
fn not_found() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: &'static str,
        message: &'static str,
    };

    let context = Context {
        title: "Unauthorized",
        message: "The request resource could not be found.",
    };

    Template::render("error", &context)
}

fn ignite() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", pages::routes())
        .register(catchers![not_found])
}

fn main() {
    ignite().launch();
}
