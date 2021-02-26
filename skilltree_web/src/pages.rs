use skilltree_core::User;
use rocket::Route;
use skilltree_core::Database;
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
    let embed = vec!["2wcw_O_19XQ".to_string(), "VjiH3mpxyrQ".to_string()];

    #[derive(Serialize)]
    struct Context {
        skill: String,
        embed: Vec<String>,
    }
    let context = Context { skill, embed };
    Template::render("skill", &context)
}

#[cfg(test)]
mod test {
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
