mod user;
mod gym;

pub use user::User;
pub use gym::Gym;
pub use gym::Tab;

use sled_extensions::bincode::Tree;
use std::error::Error;
use rocket::State;
use pwhash::bcrypt;

pub struct Database {
    pub gyms: Tree<Gym>,
    pub demo: Vec<Tab>,
}

pub trait DatabaseExt {
    fn create_gym(&mut self, name: String, pw: String, tabs_path: String) -> Result<(), Box<dyn Error>>;
    fn get_gym_url(&self, name: String) -> Result<Option<Gym>, Box<dyn Error>>;
    fn get_gyms(&self) -> Vec<Gym>;
    fn verify_gym(&self, url: String, pw: String) -> bool;
    fn add_user(&self, url: String, user: User) -> Result<(), Box<dyn Error>>;
    fn remove_user(&self, gymurl: String, username: String) -> Result<(), Box<dyn Error>>;
}

impl DatabaseExt for State<'_, Database> {
    fn create_gym(&mut self, name: String, pw: String, tabs_path: String) -> Result<(), Box<dyn Error>> {
        let gym = Gym::new(name, pw, tabs_path);
        self.gyms.insert(gym.url.clone().as_bytes(), gym)?;
        Ok(())
    }
    fn get_gym_url(&self, url: String) -> Result<Option<Gym>, Box<dyn Error>> {
        Ok(self.gyms.get(url.as_bytes())?)
    }
    fn get_gyms(&self) -> Vec<Gym> {
        self.gyms.iter().map(|x| x.unwrap().1).collect()
    }
    fn verify_gym(&self, url: String, pw: String) -> bool {
        let hash = match self.gyms.get(url.as_bytes()).unwrap() {
            Some(x) => x.pwhash,
            None => "".to_string(),
        };
        bcrypt::verify(&pw, &hash)
    }
    fn add_user(&self, gymurl: String, user: User) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.get(gymurl.as_bytes())?.unwrap();

        let mut users = self.gyms.get(gymurl.as_bytes())?.unwrap().users;
        users.push(user);

        gym.users = users;

        self.gyms.insert(gymurl.as_bytes(), gym)?;
        Ok(())
    }
    fn remove_user(&self, gymurl: String, username: String) -> Result<(), Box<dyn Error>> {
        let mut gym = self.gyms.get(gymurl.as_bytes())?.unwrap();
        gym.users = gym.users.into_iter().filter(|x| x.name != username).collect::<Vec<_>>();
        dbg!(&gym);
        self.gyms.insert(gymurl.as_bytes(), gym)?;
        Ok(())
    }
}
