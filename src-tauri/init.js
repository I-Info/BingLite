window.addEventListener('DOMContentLoaded', () => {
    // Intercept document wheel events 
    // to prevent Bing's over-scrolling behavior.
    document.addEventListener("wheel", (e) => {
        e.stopPropagation()
        return false
    })
})