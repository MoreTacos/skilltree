use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

type Hash = String; // the userhash
type Url = String; // the tab's url

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub hash: Hash,
    pub skills: HashMap<String, usize>,
    pub athletes: Vec<Hash>,
    pub tabs_package_url: String,
}

impl User {
    pub fn new(
        name: String,
        skills: HashMap<String, usize>,
        athletes: Vec<Hash>,
        tabs_package_url: String,
    ) -> Self {
        let mut hash: String = bcrypt::hash(&name)
            .unwrap()
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        hash.truncate(7);
        User {
            name,
            hash,
            skills,
            athletes,
            tabs_package_url,
        }
    }
    pub fn rename(&mut self, rename: &str) {
        self.name = rename.to_string();
    }
    pub fn is_athlete(&self) -> bool {
        !self.skills.is_empty()
    }
    pub fn is_coach(&self) -> bool {
        !self.athletes.is_empty()
    }
    pub fn insert(&mut self, skill: &str, level: usize) {
        self.skills.insert(skill.into(), level);
    }
    pub fn push(&mut self, athlete: Hash) {
        self.athletes.push(athlete)
    }
}
