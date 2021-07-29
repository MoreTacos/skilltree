use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Skill {
    pub url: String,
    pub content: String,
}

impl Skill {
    pub fn load_all(docs: &str) -> Vec<Skill> {
        let url = docs.to_string() + "skills";
        let skills: Vec<Skill> = reqwest::blocking::get(url).unwrap().json().unwrap();
        skills
    }
}


