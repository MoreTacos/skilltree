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
    let data = {
        name: name,
    };
    fetch('/add-user', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(data),
    }).then(response => response.json()).then(_ => {
        window.location.reload();
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

    let userlink = `${baseurl}/user?u=${id}`;

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
