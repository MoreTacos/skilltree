mod gym;
mod package;
mod user;
mod skill;

pub use gym::Gym;
pub use package::Package;
pub use skill::Skill;
pub use user::User;
pub use package::Tab;

use pwhash::bcrypt;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use rocket::Outcome::Success;
use rocket::State;
use sled::IVec;
use sled::Tree;
use std::error::Error;

pub struct Database {
    pub gyms: Tree,
    pub users: Tree,
    pub packages: Vec<Package>,
    pub skills: Vec<Skill>,
}

impl Database {
    pub fn new(gyms: Tree, users: Tree, docs: &str) -> Self {
        let packages = Package::load_all(docs);
        let skills = Skill::load_all(docs);
        Database {
            gyms,
            users,
            packages,
            skills,
        }
    }
}

pub trait DatabaseExt {
    fn create_gym(&mut self, name: &str, gymemail: &str, pw: &str) -> Result<Gym, Box<dyn Error>>;
    fn get_gym(&self, gymemail: &str) -> Result<Option<Gym>, Box<dyn Error>>;
    fn verify_gym(&self, gymemail: &str, pw: &str) -> Result<Option<bool>, Box<dyn Error>>;
    fn add_user(&mut self, name: &str, gymemail: &str) -> Result<User, Box<dyn Error>>;
    fn get_user(&self, userurl: &str) -> Result<Option<User>, Box<dyn Error>>;
    fn get_gym_users(&self, gymemail: &str) -> Result<Vec<User>, Box<dyn Error>>;
    fn update_user_skill(
        &mut self,
        userurl: &str,
        skill: &str,
        value: usize,
    ) -> Result<Option<User>, Box<dyn Error>>;
    fn get_packages(&self) -> Vec<Package>;
    fn get_package(&self, packageurl: &str) -> Package;
    fn get_tab(&self, packageurl: &str, taburl: &str) -> Tab;
    fn update_user_package(&mut self, userurl: &str, packageurl: &str) -> Result<Option<User>, Box<dyn Error>>;
    fn remove_user(&mut self, userurl: &str) -> Result<Option<User>, Box<dyn Error>>;
    fn get_skill(&self, skill: &str) -> Skill;
}

impl DatabaseExt for State<'_, Database> {
    fn create_gym(&mut self, name: &str, gymemail: &str, pw: &str) -> Result<Gym, Box<dyn Error>> {
        let gym: Gym = Gym::new(name, gymemail, pw);
        self.gyms.insert(gymemail, gym.clone())?;
        Ok(gym)
    }
    fn get_gym(&self, gymemail: &str) -> Result<Option<Gym>, Box<dyn Error>> {
        let gym: Option<Gym> = self.gyms.get(gymemail)?.map(|b: IVec| Gym::from(b));
        Ok(gym)
    }
    fn verify_gym(&self, gymemail: &str, pw: &str) -> Result<Option<bool>, Box<dyn Error>> {
        let verified: Option<bool> = self
            .get_gym(gymemail)?
            .map(|g: Gym| bcrypt::verify(&pw, &g.pwhash));
        Ok(verified)
    }
    fn add_user(&mut self, name: &str, gymemail: &str) -> Result<User, Box<dyn Error>> {
        let user: User = User::new(name, gymemail);
        self.users.insert(&user.userurl, user.clone())?;
        Ok(user)
    }
    fn get_user(&self, userurl: &str) -> Result<Option<User>, Box<dyn Error>> {
        let user: Option<User> = self.users.get(userurl)?.map(|b: IVec| User::from(b));
        Ok(user)
    }
    fn get_gym_users(&self, gymemail: &str) -> Result<Vec<User>, Box<dyn Error>> {
        let users: Vec<User> = self
            .users
            .iter()
            .map(|kv| User::from(kv.unwrap().1))
            .filter(|u| {
                u.gymemail == gymemail
            })
            .collect();
        Ok(users)
    }
    fn update_user_skill(
        &mut self,
        userurl: &str,
        skill: &str,
        value: usize,
    ) -> Result<Option<User>, Box<dyn Error>> {
        match self.users.remove(userurl)? {
            Some(user) => {
                let mut user: User = User::from(user);
                user.skills.insert(skill.into(), value);
                self.users.insert(userurl, user.clone())?;
                Ok(Some(user))
            }
            None => panic!("Updating user that doesn't exist"),
        }
    }
    fn get_packages(&self) -> Vec<Package> {
        self.packages.clone()
    }
    fn get_package(&self, packageurl: &str) -> Package {
        self.get_packages().into_iter().find(|x| x.packageurl == packageurl).unwrap()
    }
    fn get_tab(&self, packageurl: &str, taburl: &str) -> Tab {
        self.get_package(packageurl).tabs.into_iter().find(|x| x.taburl == taburl).unwrap()
    }
    fn update_user_package(&mut self, userurl: &str, packageurl: &str) -> Result<Option<User>, Box<dyn Error>> {
        match self.users.remove(userurl)? {
            Some(user) => {
                let mut user: User = User::from(user);
                user.packageurl = packageurl.to_string();
                self.users.insert(userurl, user.clone())?;
                Ok(Some(user))
            }
            None => panic!("Updating user that doesn't exist"),
        }
    }
    fn remove_user(&mut self, userurl: &str) -> Result<Option<User>, Box<dyn Error>> {
        let user: Option<User> = self.users.remove(userurl)?.map(|b: IVec| User::from(b));
        Ok(user)
    }
    fn get_skill(&self, skill: &str) -> Skill {
        self.skills.clone().into_iter().find(|x| x.url == skill).unwrap().clone()
    }
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
                db.unwrap().get_gym(&email).unwrap().unwrap()
            })
            .or_forward(())
    }
}
