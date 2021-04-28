use super::user::User;
use std::fs;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Tab {
    name: String,
    path: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Package {
    name: String,
    tabs: Vec<Tab>
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
        let packages = fs::read_dir("./templates/src").unwrap().map(|dir| {
            let dir = dir.unwrap().path();
            let name = dir.file_stem().unwrap().to_str().unwrap().to_string();
            let tabs = fs::read_dir(dir).unwrap().map(|tab| {
                let tab = tab.unwrap();
                let name = tab.path().file_stem().unwrap().to_str().unwrap().to_string();
                let path = tab.path().to_str().unwrap().to_string();
                Tab {
                    name,
                    path,
                }
            }).collect();
            Package { 
                name, 
                tabs 
            }
        }).collect();
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
