use std::fs;
use std::io;

pub struct Tree {
    svg: String,
    skills: Vec<String>,
}

impl Tree {
    pub fn new(path: &str) -> Self {
        // READING FILE
        let mut svg = fs::read_to_string(path).expect("Failed at reading file");

        // Remove those damn spans

        svg = svg.replace(r"<span>", "");
        svg = svg.replace(r"</span>", "");

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

        let mut sliced = svg.split("rect");

        // Removing the first slice, which is irrelevant
        let mut svg = sliced.next().unwrap().to_string();

        let sliced: Vec<_> = sliced.collect();

        let mut skills: Vec<String> = vec![];

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

            // add input after skill
            let replace = r###"<a href="/skill/"###.to_string() 
                + &skill
                + r###"">"###
                + &skill_exact
                + r###"</a><input type="range" onchange="fetch(`/api/{{username}}/"###
                + &skill
                + r###"/${this.value}`, { method: 'PUT' })" 
                oninput="this.closest('g').previousElementSibling.style.fill = `rgb(175, ${this.value}, 25)`" 
                min="0" max="255" value="{{#if skills."###
                + &skill
                + r###"}}{{skills."###
                + &skill
                + r###"}}{{else}}0{{/if}}" class="slider">"###;
            let slice = &slice.replacen(&skill_exact, &replace, 1);

            // replace fill in slice
            let find = r###"fill="#cce5ff""###;
            let replace =
                r###"fill="rgb(175, {{#if skills."###.to_string()
                + &skill
                + r###"}}{{skills."### 
                + &skill 
                + r###"}}{{else}}0{{/if}}, 25)""###;

            "{{#if method}}{{method}}{{else}}POST{{/if}}";

            let slice = &slice.replace(&find, &replace);

            // add skill to vector
            skills.push(skill.clone());

            svg = format!("{}rect {}", &svg, &slice);
        }

        Tree {
            svg,
            skills,
        }
    }

    pub fn write(self, path_tree: &str) -> io::Result<()> {
        let svg = self.svg;

        fs::write(path_tree, svg)?;

        Ok(())
    }

    pub fn print(self) -> () {
        let svg = self.svg;
        let skills = self.skills.join("---");

        println!("{}", svg);
        println!("{}", skills);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn print_small_tree() {
        let tree = Tree::new("./static/smalltree.svg");
        assert!(!tree.svg.is_empty());
        assert!(!tree.skills.is_empty());

        tree.print();
    }

    #[test]
    fn print_full_tree() {
        let tree = Tree::new("./static/fulltree.svg");
        assert!(!tree.svg.is_empty());
        assert!(!tree.skills.is_empty());

        tree.print();
    }
}
