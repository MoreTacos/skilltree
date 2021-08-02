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
use std::{thread, time};

pub fn sudo() -> Vec<Route> {
    routes![create_gym, sync]
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

#[post("/sync")]
fn sync(mut db: State<Database>) -> status::Accepted<String> {
    thread::sleep(time::Duration::from(time::Duration::from_secs(5)));
    db.sync_docs();
    status::Accepted(Some("Sync database".to_string()))
}
