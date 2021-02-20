use rocket_contrib::templates::Template;
use serde::Serialize;

#[get("/test")]
fn index() -> Template {
    #[derive(Serialize)]
    struct Context {
        title: String,
    };

    let context = Context {
        title: "TEST".to_string(),
    };

    Template::render("index", &context)
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
    ]
}
