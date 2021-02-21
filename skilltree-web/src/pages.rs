use rocket_contrib::templates::Template;
use serde::Serialize;
use rocket::Route;

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: String,
        value: String,
    };

    let context = Context {
        title: "T".to_string(),
        value: "100".to_string(),
    };

    Template::render("index", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
    ]
}
