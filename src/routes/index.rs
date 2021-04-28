use crate::core::Database;
use crate::core::DatabaseExt;
use crate::core::Gym;
use crate::core::User;
use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::Form;
use rocket::request::{self, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket::Route;
use rocket::State;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

pub fn index() -> Vec<Route> {
    routes![home, discover, join]
}

#[get("/", rank = 2)]
fn home() -> Template {
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
