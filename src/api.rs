use rocket::Route;
use rocket::State;
use pwhash::bcrypt;
use crate::core::Database;
use crate::core::Gym;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket::response::status;
use crate::core::DatabaseExt;

pub fn routes() -> Vec<Route> {
    routes![create_gym, read_gyms]
}

const H: &'static str = "$2y$05$bvIG6Nmid91Mu9RcmmWZfO5HJIMCT8riNW0hEp8f6/FuA2/mHZFpe";

#[derive(FromForm)]
struct Auth {
    name: String,
    pw: String,
    globalpw: String,
}

#[post("/create_gym", data = "<auth>")]
fn create_gym(mut db: State<Database>, auth: Form<Auth>) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    let globalpw = auth.globalpw.clone();
    let pw = auth.pw.clone();
    let name = auth.name.clone();
    if bcrypt::verify(&pw, H) {
        db.create_gym(name.clone(), pw,"./templates/src".to_string()).unwrap();
        Ok(status::Accepted(Some(format!("Successfully created gym {}", &name))))
    } else {
        Err(status::Unauthorized(Some(format!("Wrong password. {} could not be created.", &name))))
    }
}

#[get("/read_gyms")]
fn read_gyms(db: State<Database>) -> Json<Vec<Gym>> {
    dbg!(db.get_gyms());
    Json(db.get_gyms())
}

#[cfg(test)]
mod test {
    #[test]
    fn dumb() {
        assert_eq!(1, 1);
    }
}
