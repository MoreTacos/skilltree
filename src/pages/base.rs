use rocket::Route;
use crate::core::Database;
use crate::core::Gym;
use crate::core::DatabaseExt;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket::http::{Cookie, Cookies};
use rocket::response::{Flash, Redirect};

pub fn routes() -> Vec<Route> {
    routes![index, gym, login, demo, help, conduct, privacy]
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let mut context = Context::new();
    let gyms: Vec<Gym> = db.get_gyms();
    context.insert("gyms", &gyms);
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
fn login(db: State<Database>) -> Template {
    let mut context = Context::new();
    Template::render("login", &context)
}

#[post("/login")]
fn login_token(db: State<Database>, mut cookies: Cookies) -> Flash<Redirect> {
    cookies.add_private(Cookie::named("login"));
    Flash::success(Redirect::to("/"), "Successfully logged in.")
}

#[get("/demo")]
fn demo(db: State<Database>) -> rocket::response::content::Html<String> {
    let mut context = Context::new();
    let gym = Gym::new("demo".to_string(), "".to_string(),"./templates/src".to_string());
    context.insert("tabs", &gym.tabs.clone());
    context.insert("username", "Demo");
    context.insert("userhash", "");
    let mut tera = Tera::default();
    tera.add_template_file("./templates/layout.html.tera", Some("layout")).unwrap();
    tera.add_template_file("./templates/demo.html.tera", Some("user")).unwrap();
    let tabs: Vec<(&str, &str)> = gym.tabs.iter().map(|tab| (tab.url.as_str(), tab.svg.as_str())).collect();
    dbg!(gym.tabs.iter().map(|tab| tab.url.clone()).collect::<Vec<String>>());
    tera.add_raw_templates(tabs).unwrap();
    rocket::response::content::Html(tera.render("fxtree", &context).unwrap())
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
