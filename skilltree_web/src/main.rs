#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::config::Environment;
use rocket::response::status;
use rocket::State;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use serde::Deserialize;
use serde::Serialize;
use skilltree_svg::Tree as SvgTree;
use sled_extensions::bincode::Tree;
use sled_extensions::DbExt;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    skills: HashMap<String, usize>,
}

struct Database {
    users: Tree<User>,
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
    let embed = vec!["2wcw_O_19XQ".to_string(), "VjiH3mpxyrQ".to_string()];

    #[derive(Serialize)]
    struct Context {
        skill: String,
        embed: Vec<String>,
    }
    let context = Context { skill, embed };
    Template::render("skill", &context)
}

#[post("/add-user/<username>")]
fn add_user(db: State<Database>, username: String) -> status::Accepted<String> {
    let username = username.to_string();
    let mut skills = HashMap::new();

    let skill_list: Vec<String> = fs::read_to_string("./src/skills")
        .unwrap()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
    for skill in skill_list {
        skills.insert(skill, 0);
    }
    skills.insert("front-roll".to_string(), 0);
    let user = User {
        username: username.clone(),
        skills,
    };

    db.users
        .insert(username.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[put("/<username>/<skill>/<value>")]
fn update_user(
    db: State<Database>,
    username: String,
    skill: String,
    value: usize,
) -> status::Accepted<String> {
    let mut user = db.users.get(&username.as_bytes()).unwrap().unwrap();
    user.skills.insert(skill, value);
    db.users
        .insert(username.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[delete("/remove-user/<username>")]
fn delete_user(db: State<Database>, username: String) -> status::Accepted<String> {
    db.users.remove(&username.as_bytes()).unwrap().unwrap();
    status::Accepted(Some(format!("User {} updated successfully", &username)))
}

fn svg_setup() -> () {
    let src_path;
    let write_path_tree;
    let write_path_skills;

    match Environment::active().expect("config error") {
        Environment::Development => {
            src_path = "./templates/src/smalltree.svg".to_string();
            write_path_tree = "./templates/dev_templates/tree.svg.hbs".to_string();
            write_path_skills = "./src/skills".to_string();
        }
        Environment::Staging | Environment::Production => {
            src_path = "./templates/src/fulltree.svg".to_string();
            write_path_tree = "./templates/prod_templates/tree.svg.hbs".to_string();
            write_path_skills = "./src/skills".to_string();
        }
    }
    let tree = SvgTree::new(&src_path);
    tree.write(&write_path_tree, &write_path_skills).unwrap();
}

fn ignite() -> rocket::Rocket {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Database {
            users: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index, user, admin, skill, help, conduct, privacy])
        .mount("/api", routes![add_user, update_user, delete_user])
}

fn main() {
    svg_setup();

    ignite().launch();
}

#[cfg(test)]
mod test {
    use super::ignite;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn big_test1() {
        let client = Client::new(ignite()).expect("valid rocket instance");
        let response0 = client.get("/").dispatch();
        //let response1 = client.post("/add-user/TestUser").dispatch();
        //let response2 = client.get("/user/TestUser").dispatch();
        //let response3 = client.put("/api/TestUser/diveroll/0").dispatch();
        //let response4 = client.delete("/remove-user/TestUser").dispatch();
        assert_eq!(response0.status(), Status::Ok);
        //assert_eq!(response1.status(), Status::Ok);
        //assert_eq!(response2.status(), Status::Ok);
        //assert_eq!(response3.status(), Status::Ok);
        //assert_eq!(response4.status(), Status::Ok);
    }
}
