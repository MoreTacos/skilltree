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
use serde::Serialize;

pub fn user() -> Vec<Route> {
    routes![user_forward, user_index]
}

#[get("/user?<g>&<u>", rank = 2)]
fn user_forward(g: String, u: String) -> Redirect {
    Redirect::to(format!("/user?g={}&u={}&s={}", g, u, "fx"))
}

#[get("/user?<g>&<u>&<s>")]
fn user_index(db: State<Database>, g: String, u: String, s: String) -> rocket::response::content::Html<String> {
    let user = db.get_user(&g, &u);
    let tab = db.get_tab(&g, &s);

    let mut tera = Tera::default();
    tera.add_template_file("./templates/layout.html.tera", Some("layout")).unwrap();
    tera.add_template_file("./templates/user.html.tera", Some("user")).unwrap();
    tera.add_raw_template(&s, &tab.svg).unwrap();

    let mut context = Context::new();
    context.insert("username", &user.name);
    context.insert("userhash", &u);
    context.insert("gymurl", &g);
    println!("{}", &g);
    #[derive(Serialize, Debug)]
    struct DisplayTab {
        name: String,
        url: String,
    }
    let tabs: Vec<_> = db.get_user_tabs(&g, &u).into_iter().map(|x| DisplayTab{ name: x.name, url: x.url }).collect();
    context.insert("tabs", &tabs);

    context.insert("skills", &user.skills.clone());

    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}
