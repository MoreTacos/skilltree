use std::process::Command;
use std::fs;
use crate::core::Database;
use crate::core::DatabaseExt;
use crate::core::Gym;
use pwhash::bcrypt;
use rocket::data::Data;
use rocket::request::Form;
use rocket::response::status;
use rocket::Route;
use rocket::State;
use rocket_contrib::json::Json;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;

pub fn sudo() -> Vec<Route> {
    routes![create_gym, insert_page, insert_tree]
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
        db.create_gym(&name, &email, &pw).unwrap();
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

#[post("/insert_page?<n>", data = "<data>")]
fn insert_page(n: String, data: Data) -> status::Accepted<String> {
    data.stream_to_file(format!("./templates/pages/{}", n)).unwrap();
    status::Accepted(Some("Success".to_string()))
}

#[post("/insert_tree?<p>&<n>", data = "<data>")]
fn insert_tree(p: String, n: String, data: Data) -> status::Accepted<String> {
    fs::create_dir_all(format!("./templates/packages/{}", p)).unwrap();
    data.stream_to_file(format!("./templates/packages/{}/{}", p, n)).unwrap();
    status::Accepted(Some("Success".to_string()))
}
