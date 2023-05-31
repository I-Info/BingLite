import { open as shellOpen } from "@tauri-apps/plugin-shell"

console.log("Initialize scripts loaded.");

function clickListener(e: Event) {
    var target = e.target;
    while (target && (target as Element).tagName !== "A") {
        target = (target as Element).parentElement;
    }
    if (target) {
        let href = (target as HTMLAnchorElement).href;
        // Whether the href is a URL.
        if (href.startsWith("http://") || href.startsWith("https://")) {
            shellOpen(href);
            
            e.preventDefault();
            e.stopPropagation();
            return false;
        }
    }
    return true;
}

function findAllShadowRoots(node: Element, action: (root: ParentNode) => void): ShadowRoot[] {
    // Initialize an empty array to store the shadow roots
    let shadowRoots: ShadowRoot[] = [];
    // Define a helper function that recursively searches for shadow roots
    function findShadowRoots(node: Element) {
        // If the node has a shadow root, add it to the array
        if (node.shadowRoot) {
            action(node.shadowRoot);
            shadowRoots.push(node.shadowRoot);
            node.shadowRoot.childNodes.forEach(child => {
                if (child.nodeType === Node.ELEMENT_NODE) {
                    findShadowRoots((child) as Element);
                }
            })
        }
        action(node);
        // Loop through the node's child nodes
        node.childNodes.forEach(child => {
            // If the child is an element node, recursively search its tree
            if (child.nodeType === Node.ELEMENT_NODE) {
                findShadowRoots((child) as Element);
            }
        })
    }
    // Start the search from the given node
    findShadowRoots(node);
    // Return the array of shadow roots
    return shadowRoots;
}

document.addEventListener("DOMContentLoaded", () => {
    // Intercept document wheel events
    // to prevent Bing's over-scrolling behavior.
    document.addEventListener("wheel", (e) => {
        e.stopPropagation()
        return false
    })

    const action = (root: ParentNode) => {
        observer.observe(root, config);
        root.querySelectorAll("a").forEach(a => {
            a.addEventListener("click", clickListener);
        })
    }

    const config: MutationObserverInit = { attributes: false, childList: true, subtree: false }; // specify what kinds of changes to observe
    const callback: MutationCallback = (mutations: MutationRecord[], _observer: MutationObserver) => {
        mutations.forEach(mutation => {
            findAllShadowRoots(mutation.target as Element, action);
        })
    };
    const observer = new MutationObserver(callback); // create an observer instance

    // Wait for shadow root mounted.
    setTimeout(() => {
        const target = document.body.getElementsByClassName("cib-serp-main")[0];
        findAllShadowRoots(target, action);
        if (target.shadowRoot) {
            // Remove feedback box
            target.shadowRoot.querySelectorAll("cib-serp-feedback").forEach(e => {
                e.remove();
            })
        }
    }, 1000);

    console.log("Initialization finished.");
})