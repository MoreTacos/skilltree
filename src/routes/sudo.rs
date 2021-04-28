use crate::core::Database;
use crate::core::parsetab;
use crate::core::DatabaseExt;
use crate::core::Gym;
use pwhash::bcrypt;
use std::error::Error;
use rocket::request::Form;
use rocket::response::status;
use rocket::Route;
use rocket::State;
use serde::Deserialize;
use serde::Serialize;
use rocket::data::Data;
use rocket_contrib::json::Json;
use std::collections::HashMap;

pub fn sudo() -> Vec<Route> {
    routes![create_gym, create_package]
}

const H: &'static str = "$2y$05$bvIG6Nmid91Mu9RcmmWZfO5HJIMCT8riNW0hEp8f6/FuA2/mHZFpe";

#[derive(FromForm)]
struct Auth {
    name: String,
    email: String,
    pw: String,
    globalpw: String,
}

#[post("/create_gym", data = "<auth>")]
fn create_gym(
    mut db: State<Database>,
    auth: Form<Auth>,
) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    let name = auth.name.clone();
    let email = auth.email.clone();
    let pw = auth.pw.clone();
    let globalpw = auth.globalpw.clone();
    if bcrypt::verify(&globalpw, H) {
        db.create_gym(&name, &email, &pw)
            .unwrap();
        Ok(status::Accepted(Some(format!(
            "Successfully created gym {}",
            &name
        ))))
    } else {
        Err(status::Unauthorized(Some(format!(
            "Wrong password. {} could not be created.",
            &name
        ))))
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Package {
    name: String,
    globalpw: String,
    files: HashMap<String, String>,
} 

#[post("/create_package", format = "json", data = "<package>")]
fn create_package(package: Json<Package>) -> Result<status::Accepted<String>, status::Unauthorized<String>> {
    let name = package.name.clone();
    let globalpw = package.globalpw.clone();
    let tabs = package.files.clone();

    if bcrypt::verify(&globalpw, H) {
        for tab in tabs {
            parsetab(&tab.0, &name, &tab.1).unwrap();
        }
        Ok(status::Accepted(Some(format!(
            "Successfully created tab package {}",
            &name
        ))))
    } else {
        Err(status::Unauthorized(Some(format!(
            "Wrong password. {} tab package could not be created.",
            &name
        ))))
    }
}
