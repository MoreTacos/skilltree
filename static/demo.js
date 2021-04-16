let meta = document.getElementById("usermeta");
let userhash = meta.attributes[1].nodeValue || "";

let rects = document.getElementsByClassName("skill");
Array.from(rects).forEach(function (rect) {
  let skill = rect.id;

  fetch(`/api/demo`)
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
