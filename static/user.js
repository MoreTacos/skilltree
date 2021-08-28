let skills = document.getElementsByClassName("skill");
Array.from(skills).forEach(function(rect) {
    rect.setAttribute("rx", "5");
    rect.setAttribute("ry", "5");

    rect.nextSibling.classList.add("skill")
});

let width = window.width;
console.log(width);

const elem = document.getElementsByTagName("svg")[0];
const panzoom = Panzoom(elem, {
    excludeClass: 'toggle',
    maxScale: 10,
    minScale: 0.35,
    step: 0.15,
});

const parent = elem.parentElement;

parent.addEventListener('wheel', function(event) {
    panzoom.zoomWithWheel(event)
});

/*
elem.addEventListener('panzoomzoom', (event) => {
    console.log(event);
    panzoom.setOptions({ step: (event.detail.scale*0.1)  });
});
*/
