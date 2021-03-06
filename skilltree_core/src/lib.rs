extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha1::Sha1;
use std::fs;
use std::collections::HashMap;
use serde::Deserialize;
use serde::Serialize;
use sled_extensions::bincode::Tree;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Tab {
    fullname: String,
    shortname: String,
    active: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct User {
    pub username: String,
    pub userhash: String,
    pub skills: HashMap<String, usize>,
    pub tabs: Vec<Tab>,
}

pub struct Database {
    pub users: Tree<User>,
}

pub fn short_url_tabs(long_url: &str) -> String {
    let short_url = long_url
                  .split_whitespace()
                  .collect::<String>()
                  .replace(r",", "")
                  .replace(r"/", "")
                  .replace(r"\", "")
                  .replace(r"'", "")
                  .replace(r"*", "")
                  .replace(r"#", "")
                  .replace(r"$", "")
                  .replace(r".", "")
                  .replace(r"-", "")
                  .replace(r"+", "")
                  .replace(r">", "")
                  .replace(r"<", "")
                  .replace(r"}", "")
                  .replace(r"{", "")
                  .replace(r")", "")
                  .replace(r"?", "")
                  .replace(r"=", "")
                  .replace(r"^", "")
                  .replace(r"%", "")
                  .replace(r"@", "")
                  .replace(r"!", "")
                  .replace(r"~", "")
                  .replace(r"`", "")
                  .replace(r"|", "")
                  .replace(r"[", "")
                  .replace(r"]", "")
                  .replace(r"(", "")
                  .replace(r"skilltree", "")
                  .to_lowercase();
    short_url
}

impl User {
    pub fn tabs(path: &str) -> Vec<Tab> {
        let mut tabs = Vec::new();
        for entry in fs::read_dir(path).expect("failed to read path for tabs") {
            let fullname = entry.unwrap().path().file_stem().unwrap().to_str().unwrap().to_string().replace("skilltree-", "");
            let shortname = short_url_tabs(&fullname);
            let tab = Tab {
                fullname,
                shortname,
                active: true,
            };
            tabs.push(tab);
        }
        tabs
    }

    pub fn userhash(username: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.input_str(&username);
        let mut userhash = hasher.result_str();
        userhash.truncate(11);
        userhash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn userhash() {
        User::userhash("hello world");
    }

    #[test]
    fn user() {
        let username = String::from("Davide");
        let userhash = User::userhash(&username);
        let skills = HashMap::new();
        let tabs = User::tabs("./static");
        let _user = User {
            username,
            userhash,
            skills,
            tabs,
        };
    }
}
