// New User Button Functionality
function newUserButton() {
    document.getElementById('new-user-form').style.display = 'block'; // popup
    document.getElementById('bg-cover').style.display = 'block'; // darker background
}
document.getElementById('new-user-button').addEventListener("click", newUserButton);
function addUserButton() {
    document.getElementById('new-user-form').style.display = 'none';
    document.getElementById('bg-cover').style.display = 'none';
    let name = document.getElementById('name').value;
    let iscoach = document.getElementById('iscoach').value;
    if (iscoach == "on") {
        iscoach = true;
    } else {
        iscoach = false;
    }
    let tabs_select = document.getElementById('tabs');
    let tabs = tabs_select.value;
    let tab_text = tabs_select.options[tabs_select.selectedIndex].text;
    let data = {
        name: name,
        iscoach: iscoach,
        tabs_package_url: tabs,
    };
    fetch('/add-user', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
    }).then(response => response.json()).then(id => {
        let table = document.getElementById('table');
        let row = table.insertRow(1);
        let who = row.insertCell(0);
        who.classList.add("who-row");

        who.setAttribute("id", id);

        let a1 = document.createElement('a');
        a1.setAttribute("href", "#");
        let copy = document.createElement('i');
        a1.addEventListener("click", copyUserButton);
        copy.classList.add("fa");
        copy.classList.add("fa-paste");

        let a2 = document.createElement('a');
        a2.setAttribute("href", "#");
        let x = document.createElement('i');
        a2.addEventListener("click", removeUserButton);
        x.classList.add("fa");
        x.classList.add("fa-times");

        a1.appendChild(copy);
        a2.appendChild(x)

        let linkbox = document.createElement('div');
        linkbox.classList.add("linkbox");
        linkbox.appendChild(a1);
        linkbox.appendChild(a2);

        who.appendChild(linkbox);

        let p = document.createElement('p');
        p.innerHTML = name;
        who.appendChild(p);

        let athletes = row.insertCell(1);
        athletes.classList.add("athlete-row");
        athletes.innerHTML = "Empty";
        let tabs_row = row.insertCell(2);
        tabs_row.classList.add("package-row");
        tabs_row.innerHTML = tab_text;
        document.getElementById('name').value = "";
    }).catch(err => {
        console.log(err);
    });
}
document.getElementById('add-user-button').addEventListener("click", addUserButton);
function closeUserButton() {
    document.getElementById('new-user-form').style.display = 'none';
    document.getElementById('bg-cover').style.display = 'none';
    document.getElementById('name').value = "";
}
document.getElementById('close-user-button').addEventListener("click", closeUserButton);

// Remove User Button
function removeUserButton() {
    let hash = this.parentNode.parentNode.id;
    console.log(hash);
    fetch(`/remove-user`, { 
        method: 'DELETE',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({
            hash: hash,
        }),
    });
    this.parentNode.parentNode.parentNode.remove();
}
Array.from(document.getElementsByClassName("fa-times")).forEach(function(e) {
    e.parentNode.addEventListener("click", removeUserButton);
})

// Copy User Button
function copyUserButton() {
    let id = this.parentNode.parentNode.id;
    let baseurl = window.location.origin;

    let userlink = `${baseurl}/user?g=${gym}&u=${id}`;

    let el = document.createElement("textarea");
    el.value = userlink;
    document.body.appendChild(el);
    el.select();
    document.execCommand("copy");
    document.body.removeChild(el);
}
Array.from(document.getElementsByClassName("fa-paste")).forEach(function(e) {
    e.parentNode.addEventListener("click", copyUserButton);
})

/*
let elements = document.getElementsByClassName("remove-user");
Array.from(elements).forEach(function(element) {
    element.addEventListener('click', removeUser);
});
*/
