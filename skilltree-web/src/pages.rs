use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: String,
    };

    let context = Context {
        title: "TITLE".to_string(),
    };

    Template::render("index", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
    ]
}
