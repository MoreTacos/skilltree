function selectChange() {
    console.log(this.value);
    let userurl = new URLSearchParams(window.location.search).get("u");
    console.log(userurl);
    fetch(`/package?u=${userurl}&p=${this.value}`, {
        method: 'PUT'
    }).then(_ => _).then(_ => {
        window.location.reload();
    }).catch(err => {
        console.log(err);
    })
}
document.getElementById('package').addEventListener("change", selectChange);
