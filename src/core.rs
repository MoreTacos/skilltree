mod user;
mod gym;

pub use user::User;
pub use gym::Gym;

use sled_extensions::bincode::Tree;
use std::error::Error;
use rocket::State;

pub struct Database {
    pub gyms: Tree<Gym>,
}

pub trait DatabaseExt {
    fn create_gym(&mut self, name: String, tabs_path: String) -> Result<(), Box<dyn Error>>;
    fn get_gym_url(&self, name: String) -> Result<Option<Gym>, Box<dyn Error>>;
    fn get_gyms(&self) -> Vec<Gym>;
}

impl DatabaseExt for State<'_, Database> {
    fn create_gym(&mut self, name: String, tabs_path: String) -> Result<(), Box<dyn Error>> {
        let gym = Gym::new(name, tabs_path);
        self.gyms.insert(gym.url.clone().as_bytes(), gym)?;
        Ok(())
    }
    fn get_gym_url(&self, url: String) -> Result<Option<Gym>, Box<dyn Error>> {
        Ok(self.gyms.get(url.as_bytes())?)
    }
    fn get_gyms(&self) -> Vec<Gym> {
        self.gyms.iter().map(|x| x.unwrap().1).collect()
    }
}
