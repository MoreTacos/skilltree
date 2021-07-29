use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Package {
    pub packageurl: String,
    pub tabs: Vec<Tab>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Tab {
    pub taburl: String,
    pub content: String,
}

impl Package {
    pub fn load_all(docs: &str) -> Vec<Package> {
        let url = docs.to_string() + "packages";
        let packages: Vec<Package> = reqwest::blocking::get(url).unwrap().json().unwrap();
        packages
    }
}
