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
        fs::read_dir("./templates/src").unwrap().map(|packagepath| {
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

pub fn parsetab(name: &str, package: &str, svg: &str) -> Result<(), Box<dyn Error>> {
    let mut svg = svg.to_string();

    // Remove all <span> tags
    svg = svg.replace(r"<span>", "");
    svg = svg.replace(r"</span>", "");

    let mut sliced = svg.split(r"<rect");

    // Removing the first slice, which is irrelevant

    let mut svg = r###"{% extends "user" %}
{% block tree %}
"###
    .to_string()
        + sliced.next().unwrap();

    let sliced: Vec<_> = sliced.collect();

    for slice in sliced {
        let mut slice = slice.to_string();
        if slice.contains("span") {
            println!("Element containing span might not be displayed");
        }

        // find skill
        let mut search_domain = slice.to_string().clone();

        // closer to answer 1
        let from = search_domain.find("word-wrap").unwrap();
        search_domain = search_domain[from..].to_string();

        let from2 = search_domain.find(">").unwrap();
        let to = search_domain.find("<").unwrap();

        let skill_exact = search_domain[from2 + 1..to].to_string();

        let skill = skill_exact
            .split_whitespace()
            .collect::<String>()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase();

        let skill_exact_correct = slice.to_string().clone()[from..from + to].to_string();

        // skip if empty
        if skill == "".to_string() {
            println!("Skipped empty box");
            continue;
        }

        let color = "{% if skills.".to_string()
            + &skill
            + "%}{{ skills."
            + &skill
            + "}}{% else %}0{% endif %}";

        // input slider
        let onchange = format!(
            r###"fetch(`/update?u={{{{ userhash }}}}&s={}&v=${{this.value}}`, {{ method: 'PUT' }})"###,
            &skill
        );
        let oninput = r###"this.closest('g').previousElementSibling.style.fill = `hsl(${this.value}, 50%, 50%)`"###;
        let mut skill_exact_correct_with_input = skill_exact_correct.clone()
            + r###"<input type="range" min="0" max="100" value=""###
            + &color
            + r###"" onchange=""###
            + &onchange
            + r###"" oninput=""###
            + &oninput
            + r###"">"###;
        skill_exact_correct_with_input = skill_exact_correct_with_input
            .replace(&skill_exact, &format!(r"<p>{}</p>", &skill_exact));
        slice = slice.replace(&skill_exact_correct, &skill_exact_correct_with_input);

        // Skill value finder and remove (A) | (B) | (C) etc...

        let mut skillvalue: String = "".to_string();

        for c in "ABCDEFGHIabcdefghi".chars() {
            let search = format!("({})", &c);
            if slice.contains(&search) {
                skillvalue = c.to_string();
                slice = slice.replace(&search, "");
            }
        }

        svg =
            svg + r###"<rect fill="hsl("### + &color + r###", 50%, 50%)" class="skill""### + &slice;
    }

    svg = svg
        + r###"
{% endblock %}"###;

    svg = svg.replace(r"<br>", "");

    fs::create_dir_all(format!("./templates/src/{}", package)).unwrap();
    fs::write(format!("./templates/src/{}/{}", package, name), svg)?;

    Ok(())
}
