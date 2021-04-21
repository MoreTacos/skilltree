use rocket::Route;
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
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest};

pub fn routes() -> Vec<Route> {
    routes![index, discover, join]
}

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();
    Template::render("index", &context)
}

#[get("/discover")]
fn discover() -> Template {
    let mut context = Context::new();
    Template::render("discover", &context)
}

#[get("/join")]
fn join() -> Template {
    let context = Context::new();
    Template::render("join", &context)
}
