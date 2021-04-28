use super::user::User;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use std::fs;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Tab {
    pub name: String,
    pub url: String,
    pub path: String,
}

impl Tab {
    fn new(name: &str, path: &str) -> Self {
        let name = name.to_string();
        let url = name.chars().filter(|c| c.is_alphanumeric()).collect();
        let path = path.to_string();
        Tab { name, url, path }
    }
    fn content(&self) -> String {
        fs::read_to_string(self.path.clone()).unwrap()
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Package {
    pub name: String,
    pub url: String,
    pub tabs: Vec<Tab>,
}

impl Package {
    fn new(name: &str, tabs: Vec<Tab>) -> Self {
        let name = name.to_string();
        let url = name.chars().filter(|c| c.is_alphanumeric()).collect();
        Package { name, url, tabs }
    }
}

// make sure that when removing users, they are not athletes of
// anybody else in the gym! If so then remove pointer
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Gym {
    pub name: String,
    pub email: String,
    pub pwhash: String,
    pub url: String,
    pub users: Vec<User>,
    pub packages: Vec<Package>,
}

impl Gym {
    pub fn new(name: &str, email: &str, pw: &str) -> Self {
        let name = name.to_string();
        let email = email.to_string();
        let users: Vec<User> = vec![];
        let pwhash = bcrypt::hash(pw).unwrap();
        let url = name
            .clone()
            .split_whitespace()
            .collect::<String>()
            .chars()
            .filter(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c.clone()))
            .collect::<String>()
            .to_lowercase();
        let packages = fs::read_dir("./templates/src")
            .unwrap()
            .map(|dir| {
                let dir = dir.unwrap().path();
                let name = dir.file_stem().unwrap().to_str().unwrap().to_string();
                let tabs = fs::read_dir(dir)
                    .unwrap()
                    .map(|tab| {
                        let tab = tab.unwrap();
                        let name = tab
                            .path()
                            .file_stem()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        let path = tab.path().to_str().unwrap().to_string();
                        Tab::new(&name, &path)
                    })
                    .collect();
                Package::new(&name, tabs)
            })
            .collect();
        Gym {
            name,
            email,
            pwhash,
            url,
            users,
            packages,
        }
    }
}
