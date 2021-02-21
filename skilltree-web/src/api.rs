use rocket::Route;
use rocket::response::status;

#[post("/")]
fn index() -> status::Accepted<String> {
    status::Accepted(Some(format!("HELLO!")))
}

#[post("/front-roll/<value>")]
fn update(value: usize) -> status::Accepted<String> {
    println!("{} saved to the database", value);
    status::Accepted(Some(format!("value: {}", value)))
}

pub fn routes() -> Vec<Route> {
    routes![
        index,
        update,
    ]
}
