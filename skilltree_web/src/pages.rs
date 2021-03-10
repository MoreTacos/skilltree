use rocket::Route;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use skilltree_core::Database;
use skilltree_core::User;
use std::fs;

pub fn routes() -> Vec<Route> {
    routes![index, admin, help, conduct, privacy, user, skill,]
}

fn get_users(db: &State<Database>) -> Vec<User> {
    db.users
        .iter()
        .map(|user| user.expect("not user").1)
        .collect()
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let users = get_users(&db);
    let mut context = Context::new();
    context.insert("users", &users);
    Template::render("index", &users)
}

#[get("/admin")]
fn admin(db: State<Database>) -> Template {
    let users = get_users(&db);
    Template::render("admin", &users)
}

#[get("/help")]
fn help(_db: State<Database>) -> Template {
    Template::render("help", ())
}

#[get("/code-of-conduct")]
fn conduct(_db: State<Database>) -> Template {
    Template::render("conduct", ())
}

#[get("/privacy")]
fn privacy(_db: State<Database>) -> Template {
    Template::render("privacy", ())
}

#[get("/user?<u>&<s>")]
fn user(db: State<Database>, u: String, s: String) -> Template {
    let user = db.users.get(u.as_bytes()).unwrap().expect("mistake");
    Template::render(s, &user)
}

#[get("/skill/<skill>")]
fn skill(_db: State<Database>, skill: String) -> Template {
    let path = format!("./templates/skills/{}.toml", skill);
    let skill = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => fs::read_to_string("./templates/skills/missing.toml")
            .expect("failed to read missing skill file"),
    };

    let context: toml::Value = toml::from_str(&skill).unwrap();

    Template::render("skill", &context)
}

#[cfg(test)]
mod test {
    #[test]
    fn big_test1() {
        assert!(true)
    }
}
