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
use rocket::Route;
use skilltree_core::User;
use skilltree_core::Database;

pub fn routes() -> Vec<Route> {
    routes![
        add_user,
        update_user,
        delete_user,
    ]
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct User {
    username: String,
    skills: HashMap<String, usize>,
}

fn get_users_names(db: &State<Database>) -> Vec<String> {
    db.users
        .iter()
        .map(|user| user.expect("not user").1.username)
        .collect()
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

#[cfg(test)]
mod test {
    use super::ignite;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn api_test1() {
        let client = Client::new(ignite()).expect("valid rocket instance");
        let response1 = client.post("/add-user/TestUser").dispatch();
        //let response2 = client.get("/user/TestUser").dispatch();
        //let response3 = client.put("/api/TestUser/diveroll/0").dispatch();
        //let response4 = client.delete("/remove-user/TestUser").dispatch();
        //assert_eq!(response0.status(), Status::Ok);
        assert_eq!(response1.status(), Status::Ok);
        //assert_eq!(response2.status(), Status::Ok);
        //assert_eq!(response3.status(), Status::Ok);
        //assert_eq!(response4.status(), Status::Ok);
    }
}
