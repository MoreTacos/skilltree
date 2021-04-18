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
    routes![admin, add_user, remove_user, index, admin_index, gym, login, login_token, logout, demo_redirect, demo, help, conduct, privacy]
}

#[get("/", rank = 2)]
fn index() -> Template {
    let mut context = Context::new();
    context.insert("isindex", &true);
    Template::render("index", &context)
}

#[derive(Debug)]
struct Admin {
    url: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("url")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|url| Admin {
                url
            })
            .or_forward(())
    }
}

#[get("/admin")]
fn admin(db: State<Database>, admin: Admin) -> Template {
    let gym = db.get_gym_url(admin.url).unwrap().unwrap();
    dbg!(&gym);
    let mut context = Context::new();
    context.insert("isadmin", &true);
    context.insert("name", &gym.name);
    context.insert("users", &gym.users);
    Template::render("admin", &context)
}

#[post("/admin/add-user/<name>")]
fn add_user(db: State<Database>, admin: Admin, name: String) {
    let skills = HashMap::new();
    let athletes = vec![];
    let user = User::new(name, skills, athletes);
    let url = admin.url;
    db.add_user(url, user).unwrap();
}

#[delete("/admin/remove-user/<username>")]
fn remove_user(db: State<Database>, admin: Admin, username: String) {
    let gymurl = admin.url;
    db.remove_user(gymurl, username).unwrap();
}

#[get("/")]
fn admin_index(db: State<Database>, _admin: Admin) -> Template {
    let mut context = Context::new();
    let gyms: Vec<Gym> = db.get_gyms();
    context.insert("gyms", &gyms);
    context.insert("isadmin", &true);
    Template::render("index", &context)
}

#[get("/<gym>")]
fn gym(db: State<Database>, gym: String) -> Template {
    let mut context = Context::new();
    let gym = db.get_gym_url(gym).unwrap().unwrap();
    context.insert("gym", &gym);
    Template::render("gym", &context)
}

#[get("/login")]
fn login() -> Template {
    let context = Context::new();
    Template::render("login", &context)
}

#[derive(FromForm)]
struct Login {
    url: String,
    pw: String,
}

#[post("/login", data="<login>")]
fn login_token(db: State<Database>, mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    if db.verify_gym(login.url.clone(), login.pw.clone()) {
        cookies.add_private(Cookie::new("url", login.url.to_string()));
        println!("LOGIN SUCCESSFULL");
        Ok(Redirect::to("/admin"))
    } else {
        Err(Flash::error(Redirect::to("/"), "Invalid username/password."))
    }
}

#[get("/logout")]
fn logout(_admin: Admin, mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("url"));
    Redirect::to("/")
}

#[get("/demo")]
fn demo_redirect() -> Redirect {
    Redirect::to("/demo?s=fxtree")
}

#[get("/demo?<s>")]
fn demo(db: State<Database>, s: String) -> rocket::response::content::Html<String> {
    let mut context = Context::new();
    let tabs = db.demo.clone();
    context.insert("tabs", &tabs);
    context.insert("username", "Demo");
    context.insert("userhash", "");
    let mut tera = Tera::default();
    tera.add_template_file("./templates/layout.html.tera", Some("layout")).unwrap();
    tera.add_template_file("./templates/demo.html.tera", Some("user")).unwrap();
    let tabs: Vec<(&str, &str)> = tabs.iter().map(|tab| (tab.url.as_str(), tab.svg.as_str())).collect();
    tera.add_raw_templates(tabs).unwrap();
    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}

#[get("/help")]
fn help(_db: State<Database>) -> Template {
    let context = Context::new();
    Template::render("help", &context)
}

#[get("/code-of-conduct")]
fn conduct(_db: State<Database>) -> Template {
    let context = Context::new();
    Template::render("conduct", &context)
}

#[get("/privacy")]
fn privacy(_db: State<Database>) -> Template {
    let context = Context::new();
    Template::render("privacy", &context)
}

#[cfg(test)]
mod test {
    #[test]
    fn big_test1() {
        assert!(true)
    }
}
