use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

type Hash = String;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub name: String,
    pub skills: HashMap<String, usize>,
    pub athletes: Vec<Hash>,
}

impl User {
    pub fn new(name: String, skills: HashMap<String, usize>, athletes: Vec<Hash>) -> Self {
        User {
            name,
            skills,
            athletes,
        }
    }
    pub fn rename(&mut self, rename: &str) {
        self.name = rename.to_string();
    }
    pub fn hash(&self) -> Hash {
        let mut hasher = Sha1::new();
        hasher.input_str(&self.name);
        let mut hash = hasher.result_str();
        hash.truncate(11);
        hash
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
    pub fn push(&mut self, athlete: Self) {
        self.athletes.push(athlete.hash())
    }
}
