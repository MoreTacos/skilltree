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

// file upload
const fileTypes = [
    "image/svg+xml"
];

function validFileType(file) {
    return fileTypes.includes(file.type);
}

function uploadFile() {
    file = this.files[0];
    value = document.getElementById("file-upload").value;

    if (validFileType(file)) {
        file.text()
            .then(svg => {
                fetch(`/admin/upload/${value}`, { method: 'POST', body: svg })
                    .then(_ => {
                        alert("Uploaded successfully!")
                    })
                    .catch(_ => {
                        alert("Failed to upload!")
                    });
            })
            .catch(_ => {
                alert("Error!");
            });
    } else {
        alert("Wrong file type!");
    }
    //console.log(file.text());
}

document.getElementById("input-file").addEventListener('change', uploadFile);
