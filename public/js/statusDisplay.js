async function update(isAlive) {
    let statusMessage = document.querySelector("#status span");
    if (isAlive) {
        statusMessage.textContent = "available";
        statusMessage.classList.remove("offline");
        statusMessage.classList.add("available");
    }
    else {
        statusMessage.textContent = "offline";
        statusMessage.classList.remove("available");
        statusMessage.classList.add("offline");
    }
}

export { update };