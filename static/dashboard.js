// New User Button Functionality
function newUserButton() {
  document.getElementById("new-user-form").style.display = "block"; // popup
  document.getElementById("bg-cover").style.display = "block"; // darker background
}
document
  .getElementById("new-user-button")
  .addEventListener("click", newUserButton);
function addUserButton() {
  document.getElementById("new-user-form").style.display = "none";
  document.getElementById("bg-cover").style.display = "none";
  let name = document.getElementById("username").value;
  let groupurl = document.getElementById("group").value;
  console.log(group);
  let data = {
    name: name,
    groupurl: groupurl,
  };
  fetch("/add-user", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((response) => response.json())
    .then((_) => {
      window.location.reload();
    })
    .catch((err) => {
      console.log(err);
    });
}
document
  .getElementById("add-user-button")
  .addEventListener("click", addUserButton);
function closeUserButton() {
  document.getElementById("new-user-form").style.display = "none";
  document.getElementById("bg-cover").style.display = "none";
  document.getElementById("username").value = "";
}
document
  .getElementById("close-user-button")
  .addEventListener("click", closeUserButton);

// New Group Button Functionality
function newGroupButton() {
  document.getElementById("new-group-form").style.display = "block";
  document.getElementById("bg-cover").style.display = "block";
}
document
  .getElementById("new-group-button")
  .addEventListener("click", newGroupButton);

function addGroupButton() {
  document.getElementById("new-group-form").style.display = "none";
  document.getElementById("bg-cover").style.display = "none";
  let name = document.getElementById("groupname").value;
  console.log(name);
  let data = {
    name: name,
  };
  fetch("/add-group", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(data),
  })
    .then((response) => response.json())
    .then((_) => {
      window.location.reload();
    })
    .catch((err) => {
      console.log(err);
    });
}
document
  .getElementById("add-group-button")
  .addEventListener("click", addGroupButton);
function closeGroupButton() {
  document.getElementById("new-group-form").style.display = "none";
  document.getElementById("bg-cover").style.display = "none";
  document.getElementById("groupname").value = "";
}
document
  .getElementById("close-group-button")
  .addEventListener("click", closeGroupButton);

// Remove User Button
function removeUserButton() {
  let userurl = this.id;
  fetch("/remove-user", {
    method: "DELETE",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      userurl: userurl,
    }),
  })
    .then((response) => response.json())
    .then((_) => {
      window.location.reload();
    })
    .catch((err) => {
      console.log(err);
    });
}
Array.from(document.getElementsByClassName("fa-times")).forEach(function (e) {
  e.addEventListener("click", removeUserButton);
});

// Copy User Button
function copyUserButton() {
  let id = this.id;
  let baseurl = window.location.origin;

  let userlink = `${baseurl}/user?u=${id}`;

  let el = document.createElement("textarea");
  el.value = userlink;
  document.body.appendChild(el);
  el.select();
  document.execCommand("copy");
  document.body.removeChild(el);
}
Array.from(document.getElementsByClassName("fa-paste")).forEach(function (e) {
  e.addEventListener("click", copyUserButton);
});

function openMultipleLinks(event) {
  Array.from(
    event.classList
  ).forEach(function (id) {
    let baseurl = window.location.origin;

    let userlink = `${baseurl}/user?u=${id}`;
    window.open(userlink);
  });
}

/*
let elements = document.getElementsByClassName("remove-user");
Array.from(elements).forEach(function(element) {
    element.addEventListener('click', removeUser);
});
*/
