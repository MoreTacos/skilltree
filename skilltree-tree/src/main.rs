use std::fs;

fn main() {
    // READING FILE
    let tree = fs::read_to_string("./src/tree.svg")
        .expect("Failed at reading file");
    
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
    
    let mut sliced = tree.split("rect");

    // Removing the first slice, which is irrelevant
    let mut tree = sliced.next().unwrap().to_string();

    let sliced: Vec<_> = sliced.collect();

    let mut skills: Vec<String> = vec![];

    for slice in sliced {
        // find skill
        let mut search_domain = slice.to_string().clone();
        
        // closer to answer 1
        let from = search_domain.find("word-wrap").unwrap();
        search_domain = search_domain[from..].to_string();

        // closer to answer 2
        let from = search_domain.find(">").unwrap();
        let to: usize = search_domain.find("<").unwrap();

        let skill_exact: String = search_domain[from+1..to].to_string();


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
            continue
        }

        assert!(skill.clone().chars().all(char::is_alphanumeric));

        // add input after skill
        let replace = skill_exact.clone() 
                + r###"    <input type="range" onchange="fetch(`/api/{{username}}/"### 
                + &skill 
                + r###"/${this.value}`, { method: 'PUT' })" oninput="this.closest('g').previousElementSibling.style.fill = `rgb(175, ${this.value}, 25)`" min="0" max="255" value="{{skills."### 
                + &skill 
                + r###"}}" class="slider">"###;
        let slice = &slice.replace(&skill_exact, &replace);

        // replace fill in slice
        let find = r###"fill="#cce5ff""###;
        let replace = r###"fill="rgb(175, {{skills."###.to_string() + &skill + r###"}}, 25)""###;
        let slice = &slice.replace(&find, &replace);

        // add skill to vector
        skills.push(skill.clone());

        tree = format!("{}rect {}", &tree, &slice);
    }

    let skills: String = skills.join("\n");

    fs::write("tree.svg.hbs", &tree).expect("failed to write tree");
    fs::write("skills", &skills).expect("failed to write skills");
}
