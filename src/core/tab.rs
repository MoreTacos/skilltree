use std::fs;
use std::error::Error;

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

        // Color string value
        let color = format!(
            r###"{{% if skills.{} %}}{{ skills.{} }}{{% else %}}0{{% endif %}}"###,
            &skill, &skill
        );

        // input slider
        let onchange = format!(
            r###"fetch(`/api/{{{{ userhash }}}}/{}/${{this.value}}`, {{ method: 'PUT' }})"###,
            &skill
        );
        let oninput = r###"this.closest('g').previousElementSibling.style.fill = `hsl(${this.value}, 50%, 50%)`"###;
        let mut skill_exact_correct_with_input = format!(
            r###"{}<input type="range" min="0" max="100" value="{}" onchange="{}" oninput="{}">"###,
            &skill_exact_correct, &color, &onchange, &oninput
        );
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

        svg = format!(
            r###"{}<rect fill="hsl({}, 50%, 50%)" class="skill" id="{}" {}"###,
            &svg, &color, &skill, &slice
        );
    }

    svg = svg
        + r###"
{% endblock %}"###;


    fs::create_dir_all(format!("./templates/src/{}", package)).unwrap();
    fs::write(format!("./templates/src/{}/{}", package, name), svg)?;

    Ok(())
}
