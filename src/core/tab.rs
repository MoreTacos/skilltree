use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Package {
    pub name: String,
    pub url: String,
    pub packagepath: String,
    pub tabs: Vec<Tab>,
}

#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Tab {
    pub name: String,
    pub url: String,
    pub path: String,
}

impl Package {
    pub fn get_tab_path(&self, taburl: &str) -> String {
        self.tabs
            .iter()
            .find(|tab| tab.url == taburl)
            .unwrap()
            .path
            .clone()
    }
    pub fn new(packagepath: &str) -> Package {
        let packagepath = PathBuf::from(packagepath);
        let name: String = packagepath.file_stem().unwrap().to_str().unwrap().into();
        let packagepath = packagepath.to_str().unwrap().to_string();
        let url: String = name
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();
        let tabs = fs::read_dir(packagepath.clone())
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
                let url = name
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
                    .to_lowercase();
                let path = tab.path().to_str().unwrap().to_string();
                Tab { name, url, path }
            })
            .collect();
        Package { name, url, packagepath, tabs }
    }
    pub fn all() -> Vec<Package> {
        fs::read_dir("./templates/packages").unwrap().map(|packagepath| {
            let packagepath = packagepath.unwrap().path();
            let name: String = packagepath.file_stem().unwrap().to_str().unwrap().into();
            let packagepath = packagepath.to_str().unwrap().to_string();
            let url: String = name
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            let tabs = fs::read_dir(packagepath.clone())
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
                    let url = name
                        .chars()
                        .filter(|c| c.is_alphanumeric())
                        .collect::<String>()
                        .to_lowercase();
                    let path = tab.path().to_str().unwrap().to_string();
                    Tab { name, url, path }
                })
                .collect();
            Package {
                name,
                url,
                packagepath,
                tabs,
            }
        }).collect()
    }
}
