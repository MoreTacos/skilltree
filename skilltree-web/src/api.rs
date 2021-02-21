use rocket::response::status;
use rocket::Route;

#[post("/front-roll/<value>")]
fn update(value: String) -> status::Accepted<String> {
    println!("{} saved to the database", value);
    let config = sled::Config::default().path("./database".to_owned());
    let db = config.open().expect("DB failed");
    db.compare_and_swap("front-roll", None as Option<String>, Some("0"))
        .unwrap()
        .unwrap();
    status::Accepted(Some(format!("value: {}", &value)))
}

pub fn routes() -> Vec<Route> {
    routes![update,]
}
