use rocket::Route;
use super::super::core::database::Database;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

pub fn routes() -> Vec<Route> {
    routes![index, help, conduct, privacy]
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let mut context = Context::new();
    Template::render("index", &context)
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
