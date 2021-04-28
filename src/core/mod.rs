mod gym;
mod tab;
mod user;

pub use gym::Gym;
pub use user::User;
pub use tab::parsetab;

use pwhash::bcrypt;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome::Forward;
use rocket::Outcome::Success;
use rocket::State;
use sled_extensions::bincode::Tree;
use std::error::Error;

pub struct Database {
    pub gyms: Tree<Gym>,
}

pub trait DatabaseExt {
    fn create_gym(
        &mut self,
        name: &str,
        email: &str,
        pw: &str,
    ) -> Result<(), Box<dyn Error>>;
    fn get_gym_email(&self, email: &str) -> Result<Option<Gym>, Box<dyn Error>>;
    fn get_gyms(&self) -> Vec<Gym>;
    fn verify_gym(&self, email: &str, pw: &str) -> bool;
    fn add_user(&self, email: &str, user: User) -> Result<(), Box<dyn Error>>;
    fn get_user(&self, gymurl: &str, userurl: &str) -> User;
    fn remove_user(&self, email: &str, username: &str) -> Result<(), Box<dyn Error>>;
    //fn get_tab(&self, gymurl: &str, taburl: &str) -> Tab;
    //fn get_user_tabs(&self, gymurl: &str, userurl: &str) -> Vec<Tab>;
    //fn update_user_skill(&self, gymurl: &str, userurl: &str, skill: &str, value: usize) -> Result<(), Box<dyn Error>>;
}

impl DatabaseExt for State<'_, Database> {
    fn create_gym(
        &mut self,
        name: &str,
        email: &str,
        pw: &str,
    ) -> Result<(), Box<dyn Error>> {
        let gym = Gym::new(name, email, pw);
        self.gyms.insert(gym.email.clone().as_bytes(), gym)?;
        Ok(())
    }
    fn get_gym_email(&self, email: &str) -> Result<Option<Gym>, Box<dyn Error>> {
        Ok(self.gyms.get(email.as_bytes())?)
    }
    fn get_gyms(&self) -> Vec<Gym> {
        self.gyms.iter().map(|x| x.unwrap().1).collect()
    }
    fn verify_gym(&self, email: &str, pw: &str) -> bool {
        let hash = match self.gyms.get(email.as_bytes()).unwrap() {
            Some(x) => x.pwhash,
            None => "".to_string(),
        };
        bcrypt::verify(&pw, &hash)
    }
    fn add_user(&self, email: &str, user: User) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.remove(email.as_bytes())?.unwrap();

        let mut users = gym.users.clone();
        users.push(user);

        gym.users = users;

        self.gyms.insert(email.as_bytes(), gym)?;

        Ok(())
    }
    fn get_user(&self, gymurl: &str, userurl: &str) -> User {
        self.get_gyms()
            .iter()
            .find(|&g| g.url == gymurl)
            .unwrap()
            .users
            .iter()
            .find(|&u| u.hash == userurl)
            .unwrap()
            .clone()
    }
    fn remove_user(&self, email: &str, hash: &str) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.remove(email.as_bytes())?.unwrap();

        let mut users = gym.users.clone();
        users.retain(|x| x.hash.clone() != hash.clone());

        gym.users = users;

        self.gyms.insert(email.as_bytes(), gym)?;
        Ok(())
    }
    /*
    fn get_tab(&self, gymurl: &str, taburl: &str) -> Tab {
        let tabs = self
            .get_gyms()
            .iter()
            .find(|&g| g.url == gymurl)
            .unwrap()
            .clone();
        tabs.tabs.iter().find(|&t| t.url == taburl).unwrap().clone()
    }
    fn get_user_tabs(&self, gymurl: &str, userurl: &str) -> Vec<Tab> {
        let tabs = self
            .get_gyms()
            .iter()
            .find(|&g| g.url == gymurl)
            .unwrap()
            .tabs
            .clone();
        let user = self.get_user(gymurl, userurl);
        tabs.clone().retain(|x| user.tabs.contains(&x.url));
        tabs
    }
    */
}

impl<'a, 'r> FromRequest<'a, 'r> for Gym {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        request
            .cookies()
            .get_private("email")
            .and_then(|cookie| cookie.value().parse::<String>().ok())
            .map(|email| {
                let db = match request.guard::<State<Database>>() {
                    Success(db) => Some(db),
                    _ => None,
                };
                db.unwrap().get_gym_email(&email).unwrap().unwrap()
            })
            .or_forward(())
    }
}
