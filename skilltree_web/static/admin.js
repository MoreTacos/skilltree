// user CRUD
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

document.getElementById("add-user").addEventListener("click", addUser);
let elements = document.getElementsByClassName("remove-user");
Array.from(elements).forEach(function(element) {
    element.addEventListener('click', removeUser);
});
