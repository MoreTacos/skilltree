#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod api;
mod pages;

use skilltree_core::Database;
use rocket::config::Environment;
use sled_extensions::DbExt;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use skilltree_svg::Tree as SvgTree;

#[catch(404)]
fn not_found() -> Template {
    Template::render("error", ())
}

fn svg_setup() -> () {
    let src_path;
    let write_path_tree;

    match Environment::active().expect("config error") {
        Environment::Development => {
            src_path = "./templates/src/smalltree.svg".to_string();
            write_path_tree = "./templates/dev_templates/tree.svg.hbs".to_string();
        }
        Environment::Staging | Environment::Production => {
            src_path = "./templates/src/fulltree.svg".to_string();
            write_path_tree = "./templates/prod_templates/tree.svg.hbs".to_string();
        }
    }
    let tree = SvgTree::new(&src_path);
    tree.write(&write_path_tree).unwrap();
}

fn ignite() -> rocket::Rocket {
    let db = sled_extensions::Config::default()
        .path("./database")
        .open()
        .expect("Failed to open sled DB");

    rocket::ignite()
        .attach(Template::fairing())
        .manage(Database {
            users: db
                .open_bincode_tree("users")
                .expect("failed to open user tree"),
        })
        .mount("/static", StaticFiles::from("static"))
        .mount("/", pages::routes())
        .mount("/api", api::routes())
        .register(catchers![not_found])
}

fn main() {
    svg_setup();

    ignite().launch();
}

#[cfg(test)]
mod test {
    use super::ignite;
    use rocket::local::Client;
    use rocket::http::Status;

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
