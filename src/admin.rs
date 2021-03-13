use rocket::config::Environment;
use rocket::response::status;
use rocket::Data;
use rocket::Route;
use rocket::State;
use rocket_contrib::templates::Metadata;
use skilltree_core::Database;
use skilltree_core::User;
use skilltree_svg::Tree as SvgTree;
use std::collections::HashMap;
use std::path::Path;

pub fn routes() -> Vec<Route> {
    routes![add_user, rename_user, delete_user, upload_tree,]
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

#[put("/rename-user/<username>/<rename>")]
fn rename_user(db: State<Database>, username: String, rename: String) -> status::Accepted<String> {
    let userhash = User::userhash(&username);
    let mut user = db.users.get(&userhash.as_bytes()).unwrap().unwrap();
    user.username = rename.clone();
    db.users.remove(&userhash.as_bytes()).unwrap().unwrap();
    db.users
        .insert(rename.clone().as_bytes(), user)
        .expect("Failed to rename user");
    status::Accepted(Some(format!("User {} renamed successfully", &rename)))
}

#[delete("/remove-user/<username>")]
fn delete_user(db: State<Database>, username: String) -> status::Accepted<String> {
    let userhash = User::userhash(&username);
    db.users.remove(&userhash.as_bytes()).unwrap().unwrap();
    status::Accepted(Some(format!("User {} updated successfully", &username)))
}

#[post("/upload/<tree>", data = "<paste>")]
fn upload_tree(tree: String, paste: Data) {
    let src_path;
    let write_path;

    match Environment::active().expect("config error") {
        Environment::Development => {
            src_path = "./templates/dev_templates/src";
            write_path = "./templates/dev_templates";
        }
        Environment::Staging | Environment::Production => {
            src_path = "./templates/prod_templates/src";
            write_path = "./templates/prod_templates";
        }
    }

    let filename = format!("{}/skilltree-{} Tree.svg", src_path, tree);
    paste.stream_to_file(Path::new(&filename)).expect("failed to write file");

    SvgTree::write_dir(&src_path, &write_path).unwrap();
}

#[cfg(test)]
mod test {
    #[test]
    fn dumb() {
        assert_eq!(1, 1);
    }
}
