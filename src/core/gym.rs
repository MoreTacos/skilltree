use serde::Deserialize;
use serde::Serialize;
use super::user::User;
use std::fs;
use std::path::Path;
use pwhash::bcrypt;

// make sure that when removing users, they are not athletes of
// anybody else in the gym! If so then remove pointer
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Gym {
    pub name: String,
    pub pwhash: String,
    pub url: String,
    pub users: Vec<User>,
    pub tabs: Vec<Tab>,
}

impl Gym {
    pub fn new(name: String, pw: String, tabs_path: String) -> Self {
        let users: Vec<User> = vec![];
        let pwhash = bcrypt::hash(pw).unwrap();
        let url = name.clone().split_whitespace()
            .collect::<String>()
            .chars()
            .filter(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c.clone()))
            .collect::<String>()
            .to_lowercase();
        let tabs = Tab::source_path(&tabs_path);
        Gym { name, pwhash, url, users, tabs }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Tab {
    pub name: String,
    pub url: String,
    pub svg: String,
}

impl std::fmt::Debug for Tab {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tab")
            .field("name", &self.name)
            .field("svg", &self.svg.clone().truncate(100))
            .finish()
    }
}

impl Tab {
    pub fn source_path(path: &str) -> Vec<Tab> {
        let mut tabs = vec![];
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let tab = Tab::new(path.to_str().unwrap());
            tabs.push(tab);
        }
        tabs
    }
    pub fn new(path: &str) -> Self {
        let p = Path::new(&path);
        let name: String = p.file_stem().unwrap().to_str().unwrap().into();

        let mut svg = fs::read_to_string(&path).expect("Failed at reading file");

        svg = svg.replace(r"<span>", "");
        svg = svg.replace(r"</span>", "");

        let mut sliced = svg.split(r"<rect");

        // Removing the first slice, which is irrelevant

        let mut svg = r###"{% extends "user" %}
{% block tree %}
<div id="usermeta" value="{{ userhash }}">"###
            .to_string()
            + sliced.next().unwrap();

        let sliced: Vec<_> = sliced.collect();

        for slice in sliced {
            if slice.contains("span") {
                println!("Element containing span might not be displayed");
            }

            // find skill
            let mut search_domain = slice.to_string().clone();

            // closer to answer 1
            let from = search_domain.find("word-wrap").unwrap();
            search_domain = search_domain[from..].to_string();

            let from = search_domain.find(">").unwrap();
            let to = search_domain.find("<").unwrap();

            let skill_exact = search_domain[from + 1..to].to_string();

            let skill = skill_exact
                .split_whitespace()
                .collect::<String>()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();

            // skip if empty
            if skill == "".to_string() {
                continue;
            }

            assert!(skill.clone().chars().all(char::is_alphanumeric));

            svg = format!(
                r###"{}<rect class="skill" id="{}" {}"###,
                &svg, &skill, &slice
            );
        }

        svg = svg
            + r###"</div>
{% endblock %}"###;

        let url = name.clone()
                .split_whitespace()
                .collect::<String>()
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase()
                .replace("skilltree", "");

        Tab { name, url, svg }
    }
}
