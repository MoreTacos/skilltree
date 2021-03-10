use rocket::response::status;
use rocket::Route;
use rocket::State;
use rocket_contrib::json::Json;
use skilltree_core::Database;

pub fn routes() -> Vec<Route> {
    routes![get_user, update_user,]
}

#[get("/<userhash>/<skill>")]
fn get_user(db: State<Database>, userhash: String, skill: String) -> Json<usize> {
    let user = db
        .users
        .get(&userhash.as_bytes())
        .unwrap()
        .expect("failed to find userhash in database");
    let value = user.skills.get(&skill).unwrap_or(&0);
    Json(*value)
}

#[put("/<userhash>/<skill>/<value>")]
fn update_user(
    db: State<Database>,
    userhash: String,
    skill: String,
    value: usize,
) -> status::Accepted<String> {
    let mut user = db
        .users
        .get(&userhash.as_bytes())
        .unwrap()
        .expect("Failed to find user with hash");
    user.skills.insert(skill, value);
    db.users
        .insert(userhash.clone().as_bytes(), user)
        .expect("Failed to insert user");
    status::Accepted(Some(format!("User {} modified successfully", &userhash)))
}

#[cfg(test)]
mod test {
    #[test]
    fn dumb() {
        assert_eq!(1, 1);
    }
}
