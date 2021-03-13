#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate crypto;

mod admin;
mod api;
mod pages;
mod core;

use rocket::config::Environment;
use rocket_contrib::serve::StaticFiles;
use rocket::config::Value;
use rocket_contrib::templates::Template;
use rocket_contrib::templates::Engines;
use rocket_contrib::templates::tera::ast::FunctionCall;
use sled_extensions::DbExt;
use rocket_contrib::templates::tera::Context;

#[catch(404)]
fn not_found() -> Template {
    let context = Context::new();
    Template::render("error", &context)
}

fn svg_setup() -> () {
    let src_path;
    let write_path;

    match Environment::active().expect("config error") {
        Environment::Development => {
            src_path = "./templates/dev_templates/src".to_string();
            write_path = "./templates/dev_templates".to_string();
        }
        Environment::Staging | Environment::Production => {
            src_path = "./templates/prod_templates/src".to_string();
            write_path = "./templates/prod_templates".to_string();
        }
    }
    SvgTree::write_dir(&src_path, &write_path).unwrap();
}

fn ignite() -> rocket::Rocket {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(core::Database {
            users: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/static", StaticFiles::from("static"))
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .mount("/admin", admin::routes())
        .register(catchers![not_found])
}

fn main() {
    svg_setup();

    ignite().launch();
}

#[cfg(test)]
mod test {
    use super::ignite;
    use rocket::http::Status;
    use rocket::local::Client;

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
