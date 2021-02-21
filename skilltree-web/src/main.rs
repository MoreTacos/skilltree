#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod pages;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[catch(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("error", &context)
}

fn ignite() -> rocket::Rocket {
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .register(catchers![not_found])
}

fn main() {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");
    ignite().launch();
}
