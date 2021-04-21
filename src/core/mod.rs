mod gym;
mod user;

pub use gym::Gym;
pub use gym::Tab;
pub use user::User;

use pwhash::bcrypt;
use rocket::State;
use sled_extensions::bincode::Tree;
use std::error::Error;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, Request, FromRequest};
use rocket::Outcome::Success;
use rocket::Outcome::Forward;

pub struct Database {
    pub gyms: Tree<Gym>,
    pub demo: Vec<Tab>,
}

pub trait DatabaseExt {
    fn create_gym(
        &mut self,
        name: String,
        email: String,
        pw: String,
        tabs_path: String,
    ) -> Result<(), Box<dyn Error>>;
    fn get_gym_email(&self, email: String) -> Result<Option<Gym>, Box<dyn Error>>;
    fn get_gyms(&self) -> Vec<Gym>;
    fn verify_gym(&self, email: String, pw: String) -> bool;
    fn add_user(&self, email: String, user: User) -> Result<(), Box<dyn Error>>;
    fn remove_user(&self, email: String, username: String) -> Result<(), Box<dyn Error>>;
}

impl DatabaseExt for State<'_, Database> {
    fn create_gym(
        &mut self,
        name: String,
        email: String,
        pw: String,
        tabs_path: String,
    ) -> Result<(), Box<dyn Error>> {
        let gym = Gym::new(name, email, pw, tabs_path);
        self.gyms.insert(gym.email.clone().as_bytes(), gym)?;
        Ok(())
    }
    fn get_gym_email(&self, email: String) -> Result<Option<Gym>, Box<dyn Error>> {
        Ok(self.gyms.get(email.as_bytes())?)
    }
    fn get_gyms(&self) -> Vec<Gym> {
        self.gyms.iter().map(|x| x.unwrap().1).collect()
    }
    fn verify_gym(&self, email: String, pw: String) -> bool {
        let hash = match self.gyms.get(email.as_bytes()).unwrap() {
            Some(x) => x.pwhash,
            None => "".to_string(),
        };
        bcrypt::verify(&pw, &hash)
    }
    fn add_user(&self, email: String, user: User) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.get(email.as_bytes())?.unwrap();

        let mut users = self.gyms.get(email.as_bytes())?.unwrap().users;
        users.push(user);

        gym.users = users;

        self.gyms.insert(email.as_bytes(), gym)?;
        Ok(())
    }
    fn remove_user(&self, email: String, username: String) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.get(email.as_bytes())?.unwrap();
        gym.users = gym
            .users
            .into_iter()
            .filter(|x| x.name != username)
            .collect::<Vec<_>>();
        dbg!(&gym);
        self.gyms.insert(email.as_bytes(), gym)?;
        Ok(())
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Gym {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("email")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|email| {
                let db = match request.guard::<State<Database>>() {
                    Success(db) => Some(db),
                    _ => None,
                };
                db.unwrap().get_gym_email(email).unwrap().unwrap()
            })
            .or_forward(())
    }
}
