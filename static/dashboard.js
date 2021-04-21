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
    let tabs = document.getElementById('tabs').value;
    let data = {
        name: name,
        iscoach: iscoach,
        tabs: tabs,
    };
    fetch('/add-user', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
    }).then(response => response.json()).then(_data => {
        let table = document.getElementById('table');
        let row = table.insertRow(1);
        let who = row.insertCell(0);
        who.innerHTML = name;
        let copy = document.createElement('i');
        copy.classList.add("fa");
        copy.classList.add("fa-paste");
        who.appendChild(copy);
        let athletes = row.insertCell(1);
        athletes.innerHTML = "Empty";
        let tabs_row = row.insertCell(2);
        tabs_row.innerHTML = tabs;
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

function removeUser() {
    let name = this.previousElementSibling.innerText;
    fetch(`/admin/remove-user/${name}`, { method: 'DELETE' });
    this.parentNode.remove();
}

function addUser() {
    let name = document.getElementById('name-input').value;
    fetch(`/admin/add-user/${name}`, { method: 'POST' });
    
    let li = document.createElement('li');
    let p = document.createElement('p');
    let button = document.createElement('button');
    p.innerHTML=name;
    button.addEventListener('click', removeUser);
    button.innerHTML = "Remove";
    li.appendChild(p);
    li.appendChild(button);
    let ul = document.getElementById('user-list');
    ul.appendChild(li);

    document.getElementById('name-input').value = '';
}

/*
let elements = document.getElementsByClassName("remove-user");
Array.from(elements).forEach(function(element) {
    element.addEventListener('click', removeUser);
});
*/
