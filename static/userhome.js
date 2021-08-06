function selectChange() {
    let userurl = new URLSearchParams(window.location.search).get("u");
    fetch(`/package?u=${userurl}&p=${this.value}`, {
        method: 'PUT'
    }).then(_ => _).then(_ => {
        window.location.reload();
    }).catch(err => {
        console.log(err);
    })
}
document.getElementById('package').addEventListener("change", selectChange);
