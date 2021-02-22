use std::fs;

fn main() {
    let mut tree = fs::read_to_string("./src/tree.svg")
        .expect("Failed at reading file");
    

    let mut col = tree.split("word-wrap: normal;");

    let mut tree = col.next().unwrap().to_string();

    let col: Vec<_> = col.collect();

    let mut skills: Vec<String> = vec![];

    for thing in col {
        let from: usize = thing.find(">").unwrap();
        let to: usize = thing.find("<").unwrap();
        let skill: String = thing[from+1..to].split_whitespace().collect();
        let skill = skill.to_lowercase();
        skills.push(skill.clone());
        let p1 = thing[..to].to_string();
        let p2 = thing[to..].to_string();
        let username = "{{username}}";
        let this_value = "${this.value}";

        let old = format!("skills.{}", &skill);

        let value = "{{".to_string() + &old + "}}";
        let method = "{ method: 'PUT' }";
        let input = format!(r###"<input 
            type="range" 
            onchange="fetch(`/api/{}/{}/{}`, {})" 
            oninput="this.closest('g').previousElementSibling.style.fill = `rgb(100, {}, 0)`" 
            min="0" 
            max="255" 
            value="{}" 
            class="slider">"###, &username, &skill, &this_value, &method, &this_value, &value);
        tree = format!("{} word-wrap:normal; {} {} {}", &tree, &p1, &input, &p2);
    }

    let mut col = tree.split(r###"fill="#cce5ff""###);

    let mut tree = String::new();

    let col: Vec<_> = col.collect();

    skills.push("".to_string());

    let mut i = 0;
    for thing in col {
        let mut replace = r###"fill="rgb(100, "###.to_string() + "{{skills." + &skills[i] + "}}" + r#", 0)""#;
        i+=1;
        if i > skills.len() - 1 {
            replace = "".to_string();
        }
        tree = format!("{} {} {}", &tree, &thing, &replace);
    }

    //println!("{}", &tree);
    for skill in skills {
        println!("{}", skill);
    }
}
