use std::collections::HashMap;
use rocket::response::status;
use rocket::State;
use rocket::Route;
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
    let skills = HashMap::new();

    let user = User {
        username: username.clone(),
        skills,
    };

    db.users
        .insert(username.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} added successfully", &username)))
}

#[put("/<username>/<rename>")]
fn rename_user(
    db: State<Database>,
    username: String,
    rename: String,
) -> status::Accepted<String> {
    let mut user = db.users.get(&username.as_bytes()).unwrap().unwrap();
    user.username = rename.clone();
    db.users.remove(&username.as_bytes()).unwrap().unwrap();
    db.users.insert(rename.clone().as_bytes(), user).expect("Failed to rename user");
    status::Accepted(Some(format!("User {} renamed successfully", &rename)))
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
    #[test]
    fn dumb() {
        assert_eq!(1, 1);
    }
}
