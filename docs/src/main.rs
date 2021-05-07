use glob::glob;
use pandoc::OutputKind;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let mut skills: Vec<String> = vec![];
    for path in glob("./packages/**/*.svg").expect("Failed at glob") {
        let mut svg = fs::read_to_string(path.unwrap()).unwrap();

        // Remove all <span> tags
        svg = svg.replace(r"<span>", "");
        svg = svg.replace(r"</span>", "");

        let mut sliced = svg.split(r"<rect");

        sliced.next();

        let sliced: Vec<_> = sliced.collect();

        for slice in sliced {
            let slice = slice.to_string();
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

            // skip if empty
            if skill == "".to_string() {
                println!("Skipped empty box");
                continue;
            }

            skills.push(skill);
        }
    }

    for path in fs::read_dir("./packages").unwrap() {
        let path = path.unwrap().path();
        let package = path.file_name().unwrap().to_str().unwrap().to_string();

        for path in fs::read_dir(path).unwrap() {
            let path = path.unwrap().path();
            let name = path.file_stem().unwrap().to_str().unwrap();

            let mut svg = fs::read_to_string(&path).unwrap();

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
                skill_exact_correct_with_input = skill_exact_correct_with_input.replace(
                    &skill_exact,
                    &format!(
                        r###"<p><a href="/skill?s={}">{}</a></p>"###,
                        &skill, &skill_exact
                    ),
                );
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

                svg = svg
                    + r###"<rect fill="hsl("###
                    + &color
                    + r###", 50%, 50%)" class="skill""###
                    + &slice;
            }

            svg = svg
                + r###"
{% endblock %}"###;

            svg = svg.replace(r"<br>", "");

            fs::create_dir_all(format!("../templates/packages/{}", package)).unwrap();
            fs::write(format!("../templates/packages/{}/{}", package, name), svg).unwrap();
        }
    }

    for skill in skills {
        let path = format!("./pages/{}.md", &skill);
        if !Path::new(&path).exists() {
            let default = fs::read_to_string("./default.md").unwrap();

            fs::write(&path, &default).unwrap();
        }
    }

    for path in glob("./pages/*.md").expect("failed to glob") {
        let path = path.unwrap();

        let metadata = fs::metadata(&path).unwrap();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();

        if last_modified < 60 * 30 && metadata.is_file() {
            let skill = path.file_stem().unwrap().to_str().unwrap().to_string();
            println!("Processing file: {}", &skill);

            let output = PathBuf::from(&format!("../templates/pages/{}.html", skill));
            let output1 = output.clone().to_str().unwrap().to_string();
            let output2 = format!("../templates/pages/{}", skill);

            let mut pandoc = pandoc::new();
            pandoc.add_input(&path);
            pandoc.set_output(OutputKind::File(output));
            pandoc.execute().unwrap();

            fs::rename(&output1, &output2).unwrap();

            let content = fs::read_to_string(&output2).unwrap();

            let content = r###"{% extends "docs" %}

{% block body %}
"###
            .to_string()
                + &content
                + r###"<div class="issue"><a href="https://github.com/MoreTacos/skilltree/tree/master/docs/pages/{{ skill }}.md">Add something to the page?</a></div>"###
                + r###"
{% endblock %}"###;

            fs::write(output2, content).unwrap();
        }
    }
}
