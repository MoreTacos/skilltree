use std::path::PathBuf;
use std::fs;
use std::io;
use skilltree_core::short_url_tabs;

pub struct Tree {
    svg: String,
}

impl Tree {
    fn new(path: &str) -> Self {
        // READING FILE
        let mut svg = fs::read_to_string(path).expect("Failed at reading file");

        // Remove those damn spans

        svg = svg.replace(r"<span>", "");
        svg = svg.replace(r"</span>", "");
        svg = svg.replace(r###"<?xml version="1.0" encoding="UTF-8"?>"###, "");
        svg = svg.replace(r###"<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">"###, "");

        // The general idea here is to locate the right places using split on a
        // string.
        //
        // <rect ... fill="#cce5ff" />
        // <g ...>
        //    <switch>
        //     <foreignObject ...>
        //      <div ...>
        //       <div ...>
        //        <div ...word-wrap:normal>
        //           SKILL
        //        </div>
        //      </div>
        //     </div>
        //    </foreignObject>
        //   <text ...>SKILL(not fully typed)</text>
        //  </switch>
        // </g>

        let mut sliced = svg.split(r"<rect");

        // Removing the first slice, which is irrelevant

        let mut svg = r###"{% extends "user" %}

{% block tree %}
<div id="usermeta" value="{{ userhash }}">"###.to_string() + sliced.next().unwrap();

        let sliced: Vec<_> = sliced.collect();

        for slice in sliced {
            if slice.contains("span") {
                println!("Element containing span might not be displayed");
            }

            // find skill
            let mut search_domain = slice.to_string().clone();

            // closer to answer 1
            let from = match search_domain.find("word-wrap") {
                Some(x) => x,
                None => 0,
            };
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

            svg = format!(r###"{}<rect class="skill" id="{}" {}"###, &svg, &skill, &slice);
        }
        
        svg = svg + r###"</div>
{% endblock %}"###;

        Tree { svg }
    }

    fn write_file(self, path: &str) -> io::Result<()> {
        let svg = self.svg;

        fs::write(path, svg)?;

        Ok(())
    }

    pub fn write_dir(from: &str, to: &str) -> io::Result<()> {
        for entry in fs::read_dir(from)? {
            let entry = entry?;
            let path = entry.path();
            let tree = Tree::new(path.to_str().expect("empty path"));
            let mut to_path = PathBuf::new();
            to_path.push(to);
            let short_name = short_url_tabs(entry.path().file_stem().unwrap().to_str().unwrap());
            to_path.push(format!("{}.html.tera", short_name));
            tree.write_file(&to_path.to_str().unwrap())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_tree() {
        Tree::write_dir("./static/smalltree/", "/tmp").expect("failed smalltree test");
    }

    #[test]
    fn full_tree() {
        Tree::write_dir("./static/fulltree", "/tmp/").expect("failed bigtree test");
    }
}
