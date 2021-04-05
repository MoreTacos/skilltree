let meta = document.getElementById("usermeta");
let userhash = meta.attributes[1].nodeValue || "";

let rects = document.getElementsByClassName("skill");
Array.from(rects).forEach(function (rect) {
  let skill = rect.id;

  fetch(`/api/${userhash}/${skill}`)
    .then((response) => response.text())
    .then((value) => {
      rect.style.fill = `rgb(175, ${value}, 25)`;

      let input = document.createElement("input");
      input.setAttribute('type', 'range');
      input.setAttribute('min', 0);
      input.setAttribute('max', 255);
      input.setAttribute('value', value);

      input.addEventListener("change", function () {
        fetch(`/api/${userhash}/${skill}/${input.value}`, { method: "PUT" });
      });

      input.addEventListener("input", function () {
        this.closest(
          "g"
        ).previousElementSibling.style.fill = `rgb(175, ${this.value}, 25)`;
      });

      rect.nextSibling.firstChild.firstChild.firstChild.firstChild.firstChild.appendChild(
        input
      );
    });
});

/*
            let replace = r###"<a href="/skill/"###.to_string()
                + &skill
                + r###"">"###
                + &skill_exact
                + r###"</a><input type="range" onchange="fetch(`/api/{{this.userhash}}/"###
                + &skill
                + r###"/${this.value}`, { method: 'PUT' })" 
                oninput="this.closest('g').previousElementSibling.style.fill = `rgb(175, ${this.value}, 25)`" 
                min="0" max="255" value="{{#if this.skills."###
                + &skill
                + r###"}}{{this.skills."###
                + &skill
                + r###"}}{{else}}0{{/if}}" class="slider"></input>"###;
            let slice = &slice.replacen(&skill_exact, &replace, 1);

            // replace fill in slice
            let find = r###"fill="#cce5ff""###;
            let replace = r###"fill="rgb(175, {{#if this.skills."###.to_string()
                + &skill
                + r###"}}{{this.skills."###
                + &skill
                + r###"}}{{else}}0{{/if}}, 25)""###;

            "{{#if method}}{{method}}{{else}}POST{{/if}}";
            */