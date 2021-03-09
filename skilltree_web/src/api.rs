use std::collections::HashMap;
use rocket::config::Environment;
use rocket::response::status;
use rocket::State;
use rocket::Route;
use rocket::Data;
use std::path::Path;
use skilltree_core::User;
use skilltree_core::Database;

pub fn routes() -> Vec<Route> {
    routes![
        add_user,
        rename_user,
        update_user,
        delete_user,
    ]
}

#[post("/add-user/<username>")]
fn add_user(db: State<Database>, username: String) -> status::Accepted<String> {
    let username = username.to_string();
    let userhash = User::userhash(&username);
    let skills = HashMap::new();

    let tabs_src;

    match Environment::active().expect("config error") {
        Environment::Development => {
            tabs_src = "./templates/dev_templates/src";
        }
        Environment::Staging | Environment::Production => {
            tabs_src = "./templates/prod_templates/src"
        }
    }

    let tabs = User::tabs(tabs_src); 

    let user = User {
        username: username.clone(),
        userhash: userhash.clone(),
        skills,
        tabs,
    };

    db.users
        .insert(userhash.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[put("/rename/user/<username>/<rename>")]
fn rename_user(
    db: State<Database>,
    username: String,
    rename: String,
) -> status::Accepted<String> {
    let userhash = User::userhash(&username);
    let mut user = db.users.get(&userhash.as_bytes()).unwrap().unwrap();
    user.username = rename.clone();
    db.users.remove(&userhash.as_bytes()).unwrap().unwrap();
    db.users.insert(rename.clone().as_bytes(), user).expect("Failed to rename user");
    status::Accepted(Some(format!("User {} renamed successfully", &rename)))
}

#[put("/<userhash>/<skill>/<value>")]
fn update_user(
    db: State<Database>,
    userhash: String,
    skill: String,
    value: usize,
) -> status::Accepted<String> {
    let mut user = db.users.get(&userhash.as_bytes()).unwrap().expect("Failed to find user with hash");
    user.skills.insert(skill, value);
    db.users
        .insert(userhash.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} modified successfully", &userhash)))
}

#[delete("/remove-user/<username>")]
fn delete_user(db: State<Database>, username: String) -> status::Accepted<String> {
    let userhash = User::userhash(&username);
    db.users.remove(&userhash.as_bytes()).unwrap().unwrap();
    status::Accepted(Some(format!("User {} updated successfully", &username)))
}

#[post("/upload/<name>", data = "<paste>")]
fn upload_tree(name: String, paste: Data) -> Result<(), std::io::Error> {

    let upload_target;

    match Environment::active().expect("config error") {
        Environment::Development => {
            upload_target = "./templates/dev_templates/src";
        }
        Environment::Staging | Environment::Production => {
            upload_target = "./templates/prod_templates/src"
        }
    }

    let filename = format!("{}/{} Tree", upload_target, name);
    paste.stream_to_file(Path::new(&filename))?;

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn dumb() {
        assert_eq!(1, 1);
    }
}
