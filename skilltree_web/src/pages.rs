use rocket::Route;
use rocket::State;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::tera::Context;
use rocket_contrib::templates::tera::Tera;
use rocket::config::Environment;
use skilltree_core::Database;
use skilltree_core::User;

pub fn routes() -> Vec<Route> {
    routes![index, admin, help, conduct, privacy, user]
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
    Template::render("index", &context)
}

#[get("/admin")]
fn admin(db: State<Database>) -> Template {
    let users = get_users(&db);
    let mut context = Context::new();
    context.insert("users", &users);
    Template::render("admin", &context)
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

#[get("/user?<u>&<s>")]
fn user(db: State<Database>, u: String, s: String) -> rocket::response::content::Html<String> {
    let user = db.users.get(u.as_bytes()).unwrap().expect("mistake");
    let mut context = Context::new();
    context.insert("username", &user.username);
    context.insert("userhash", &user.userhash);
    context.insert("tabs", &user.tabs);
    let mut tera = Tera::default();

    let tpath;

    match Environment::active().expect("config error") {
        Environment::Development => {
            tpath = "dev_templates".to_string();
        }
        Environment::Staging | Environment::Production => {
            tpath = "prod_templates".to_string();
        }
    }

    tera.add_template_file(format!("./templates/{}/layout.html.tera", &tpath), Some("layout")).unwrap();
    tera.add_template_file(format!("./templates/{}/user.html.tera", &tpath), Some("user")).unwrap();
    tera.add_template_file(format!("./templates/{}/{}.html.tera", &tpath, &s), Some(&s)).unwrap();
    rocket::response::content::Html(tera.render(&s, &context).unwrap())
}

/*
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
*/

#[cfg(test)]
mod test {
    #[test]
    fn big_test1() {
        assert!(true)
    }
}
