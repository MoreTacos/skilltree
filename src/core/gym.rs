use super::user::User;
use pwhash::bcrypt;
use serde::Deserialize;
use serde::Serialize;
use std::fs;
use std::path::Path;

// make sure that when removing users, they are not athletes of
// anybody else in the gym! If so then remove pointer
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct Gym {
    pub name: String,
    pub email: String,
    pub pwhash: String,
    pub url: String,
    pub users: Vec<User>,
    pub tabs: Vec<Tab>,
}

impl Gym {
    pub fn new(name: &str, email: &str, pw: &str, tabs_path: &str) -> Self {
        let users: Vec<User> = vec![];
        let pwhash = bcrypt::hash(pw).unwrap();
        let url = name
            .clone()
            .split_whitespace()
            .collect::<String>()
            .chars()
            .filter(|c| "ABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c.clone()))
            .collect::<String>()
            .to_lowercase();
        let tabs = Tab::source_path(&tabs_path);
        Gym {
            name: name.to_string(),
            email: email.to_string(),
            pwhash,
            url,
            users,
            tabs,
        }
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
            .field("url", &self.url)
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

            // Color string value
            let color = format!(r###"{{% if skills.{} %}}{{ skills.{} }}{{% else %}}0{{% endif %}}"###, &skill, &skill);

            // input slider
            let onchange = format!(r###"fetch(`/api/{{{{ userhash }}}}/{}/${{this.value}}`, {{ method: 'PUT' }})"###, &skill);
            let oninput = r###"this.closest('g').previousElementSibling.style.fill = `hsl(${this.value}, 50%, 50%)`"###;
            let mut skill_exact_correct_with_input = format!(r###"{}<input type="range" min="0" max="100" value="{}" onchange="{}" oninput="{}">"###, &skill_exact_correct, &color, &onchange, &oninput);
            skill_exact_correct_with_input = skill_exact_correct_with_input.replace(&skill_exact, &format!(r"<p>{}</p>", &skill_exact));
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



            svg = format!(
                r###"{}<rect fill="hsl({}, 50%, 50%)" class="skill" id="{}" {}"###,
                &svg, &color, &skill, &slice
            );
        }

        svg = svg
            + r###"
{% endblock %}"###;

        let url = name
            .clone()
            .split_whitespace()
            .collect::<String>()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>()
            .to_lowercase()
            .replace("skilltree", "");

        let name = name.to_uppercase();

        Tab { name, url, svg }
    }
}
