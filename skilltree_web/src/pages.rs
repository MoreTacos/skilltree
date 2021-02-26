use rocket::State;
use std::fs;
use rocket::Route;
use skilltree_core::Database;
use rocket_contrib::templates::Template;
use serde::Serialize;

pub fn routes() -> Vec<Route> {
    routes![
        index,
        admin,
        help,
        conduct,
        privacy,
        user,
        skill,
    ]
}

fn get_users_names(db: &State<Database>) -> Vec<String> {
    db.users
        .iter()
        .map(|user| user.expect("not user").1.username)
        .collect()
}

#[get("/")]
fn index(db: State<Database>) -> Template {
    let users = get_users_names(&db);
    #[derive(Serialize)]
    struct Context {
        users: Vec<String>,
    };
    let context = Context { users };
    Template::render("index", &context)
}

#[get("/admin")]
fn admin(db: State<Database>) -> Template {
    let users = get_users_names(&db);
    #[derive(Serialize)]
    struct Context {
        users: Vec<String>,
    }
    let context = Context { users };
    Template::render("admin", &context)
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

#[get("/user/<username>")]
fn user(db: State<Database>, username: String) -> Template {
    let user = db.users.get(username.as_bytes()).unwrap().unwrap();
    Template::render("user", &user)
}

#[get("/skill/<skill>")]
fn skill(_db: State<Database>, skill: String) -> Template {
    let path = format!("./templates/skills/{}.toml", skill);
    let skill = match fs::read_to_string(path) {
        Ok(v) => v,
        Err(_) => {
            fs::read_to_string("./templates/skills/missing.toml").expect("failed to read missing skill file")
        }
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
