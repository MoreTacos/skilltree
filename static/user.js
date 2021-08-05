let skills = document.getElementsByClassName("skill");
Array.from(skills).forEach(function(rect) {
    rect.setAttribute("rx", "5");
    rect.setAttribute("ry", "5");

    rect.nextSibling.classList.add("skill")
});
const elem = document.getElementsByTagName("svg")[0];
const panzoom = Panzoom(elem, {
    excludeClass: 'skill',
    maxScale: 10,
    minScale: 1.5,
    step: 0.15,
});
const parent = elem.parentElement;

parent.addEventListener('wheel', function(event) {
    panzoom.zoomWithWheel(event)
})
