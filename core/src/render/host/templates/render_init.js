(function () {
    const PREFIX = 'wayfolio:';
    const URL = 'url';
    const PASTE = 'paste';
    const FILE = 'file';
    const TAB = 'tab';

    function load(key) {
        try {
            return localStorage.getItem(PREFIX + key);
        } catch (e) {
            return null;
        }
    }

    function save(key, value) {
        try {
            localStorage.setItem(PREFIX + key, value);
        } catch (e) {
            // nothing
        }
    }

    let hash_url = null;
    try {
        hash_url = new URLSearchParams(location.hash.slice(1)).get(URL);
    } catch {
        // nothing
    }

    let saved_paste = load(PASTE);
    let saved_url = hash_url;
    if (saved_url) {
        save(URL, saved_url);
    } else {
        saved_url = load(URL);
    }

    let saved_tab;
    if (hash_url) {
        saved_tab = URL;
        save(TAB, saved_tab);
    } else {
        saved_tab = load(TAB);
        if (saved_tab !== PASTE && saved_tab !== FILE && saved_tab !== URL) {
            saved_tab = PASTE;
        }
    }

    document.documentElement.setAttribute('data-tab', saved_tab);

    if (saved_paste || saved_url) {
        let observer = new MutationObserver(function(records) {
            for (let record of records) {
                for (let node of record.addedNodes) {
                    if (saved_paste && node.id === 'paste-input') {
                        node.value = saved_paste;
                    }
                    if (saved_url && node.id === 'url-input') {
                        node.value = saved_url;
                    }
                }
            }
        });
        observer.observe(document.documentElement, {childList: true, subtree: true});
        document.addEventListener('DOMContentLoaded', function() {
            observer.disconnect();
        });
    }
})();
