use rocket::Route;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;

use super::core::Database;
use super::core::Gym;

pub fn routes() -> Vec<Route> {
    routes![index, help, conduct, privacy, login]
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let mut context = Context::new();
    let gyms: Vec<Gym> = db.gyms.iter().map(|x| x.unwrap().1).collect();
    context.insert("gyms", &gyms);
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

#[get("/login")]
fn login(_db: State<Database>) -> Template {
    let context = Context::new();
    Template::render("login", &context)
}

#[cfg(test)]
mod test {
    #[test]
    fn big_test1() {
        assert!(true)
    }
}
