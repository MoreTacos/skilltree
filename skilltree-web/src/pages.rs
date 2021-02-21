use rocket::Route;
use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    let config = sled::Config::default().path("./database".to_owned());
    let db = config.open().expect("DB failed");
    let value = db.get("front-roll").unwrap().unwrap().subslice(0, 1);
    #[derive(Serialize)]
    struct Context {
        title: String,
        value: usize,
    };

    let context = Context {
        title: "T".to_string(),
        value,
    };

    Template::render("index", &context)
}

pub fn routes() -> Vec<Route> {
    routes![index,]
}
