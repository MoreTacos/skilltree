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
use serde::Serialize;
use std::collections::HashMap;

pub fn user() -> Vec<Route> {
    routes![user_forward, user_index]
}

#[get("/user?<g>&<u>", rank = 2)]
fn user_forward(db: State<Database>, g: String, u: String) -> Redirect {
    let tabdefault = db.get_user_tab_default(&g, &u);
    Redirect::to(format!("/user?g={}&u={}&s={}", g, u, &tabdefault.url))
}

#[get("/user?<g>&<u>&<s>")]
fn user_index(
    db: State<Database>,
    g: String,
    u: String,
    s: String,
) -> rocket::response::content::Html<String> {
    let user = db.get_user(&g, &u);
    let tab = db.get_user_tab(&g, &u, &s);

    let mut tera = Tera::default();
    tera.add_template_file("./templates/layout.html.tera", Some("layout"))
        .unwrap();
    tera.add_template_file("./templates/user.html.tera", Some("user"))
        .unwrap();
    tera.add_template_file(tab.path, Some(&s)).unwrap();

    let mut context = Context::new();
    context.insert("username", &user.name);
    context.insert("userhash", &u);
    context.insert("gymurl", &g);
    let tabs: Vec<_> = db.get_user_tabs(&g, &u);
    context.insert("tabs", &tabs);

    context.insert("skills", &user.skills.clone());

    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}

#[put("/update_skill?<g>&<u>&<s>&<v>")]
fn update_user_skill(
    db: State<Database>,
    g: String,
    u: String,
    s: String,
    v: usize,
) -> rocket::response::status::Accepted<String> {
    //    db.update_user_skill(&g, &u, &s, &v).unwrap();
    rocket::response::status::Accepted(Some("Success".to_string()))
}
