console.log("Initialize scripts loaded.");

var uninitialized = true;

document.addEventListener("DOMContentLoaded", () => {
    if (uninitialized) {
        // Intercept document wheel events
        // to prevent Bing's over-scrolling behavior.
        document.addEventListener("wheel", (e) => {
            e.stopPropagation()
            return false
        })

        uninitialized = false;
        console.log("Initialization finished.");
    }
})