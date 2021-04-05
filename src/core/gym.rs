use crypto::digest::Digest;
use crypto::sha1::Sha1;
use serde::Deserialize;
use serde::Serialize;
use sled_extensions::bincode::Tree;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use sled_extensions::DbExt;
use rocket_contrib::templates::tera::Context;

// make sure that when removing users, they are not athletes of
// anybody else in the gym! If so then remove pointer
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Gym {
    pub name: String,
    pub users: Vec<User>,
    pub tabs: Vec<Tab>,
}

impl Gym {
    pub fn new(name: String, tabs_path: String) -> Self {
        let users = vec![];
        let tabs = Tab::source_path(&tabs_path);
        Gym { name, users, tabs }
    }
    pub fn url(&self) -> String {
        self.name.split_whitespace().collect::<String>().replace(r"-", "").to_lowercase()
    }
}
