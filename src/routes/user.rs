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
    Redirect::to(format!("/user?g={}&u={}&s={}", g, u, "fxtree"))
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
    context.insert("userhash", &user.hash);
    #[derive(Serialize)]
    struct MyTab {
        name: String,
        url: String,
    }
    let tabs: Vec<MyTab> = user.tabs.clone().into_iter().map(|x| MyTab{ name: x.clone(), url: x.clone() }).collect();
    context.insert("tabs", &tabs);

    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}
