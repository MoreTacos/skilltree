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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Tab {
    pub name: String,
    pub svg: String,
}

impl Tab {
    pub fn url(&self) -> String {
        self.name.split_whitespace().collect::<String>().replace(r"-", "").to_lowercase()
    }
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

        Tab { name, svg }
    }
}
