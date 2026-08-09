import {fetch_xml, generate_html} from './render_common.js';

const el_status = document.getElementById('status');
const el_output_wrap = document.getElementById('output_wrap');
const el_output = document.getElementById('output');
const el_open_link = document.getElementById('open_link');
const el_share_link = document.getElementById('share_link');
const el_paste_input = document.getElementById('paste_input');
const el_paste_render = document.getElementById('paste_render');
const el_url_input = document.getElementById('url_input');
const el_url_render = document.getElementById('url_render');
const el_file_input = document.getElementById('file_input');
const el_drop_zone = document.getElementById('drop_zone');
const el_tabs = document.querySelectorAll('.tab');

function save(key, value) {
    try {
        localStorage.setItem('wayfolio:' + key, value);
    } catch {
        // nothing
    }
}

function set_status(message, kind) {
    el_status.textContent = message;
    el_status.className = kind;
}

function make_links_absolute(html) {
    const doc = new DOMParser().parseFromString(html, 'text/html');
    for (const anchor of doc.querySelectorAll('a[href],link[href]')) {
        const href = anchor.getAttribute('href');
        if (href && !href.startsWith('#')) {
            anchor.setAttribute('href', new URL(href, window.location.href).href);
        }
    }
    return '<!doctype html>\n' + doc.documentElement.outerHTML;
}

function show_result(html, url) {
    const page = make_links_absolute(html);
    const blob_url = URL.createObjectURL(new Blob([page], {type: 'text/html'}));
    el_output.src = blob_url;
    el_output_wrap.hidden = false;
    el_open_link.href = blob_url;
    el_share_link.hidden = !url;
    if (url) {
        el_share_link.href = 'render-url.html?url=' + encodeURIComponent(url);
    }
    el_output_wrap.scrollIntoView({behavior: 'smooth', block: 'nearest'});
}

async function render(xml, url) {
    if (!xml || !xml.trim()) {
        set_status('No XML provided.', 'error');
        return;
    }
    set_status('Rendering...', 'info');
    try {
        const html = await generate_html(xml);
        show_result(html, url);
        set_status('Rendered successfully.', 'ok');
    } catch (e) {
        console.error(e);
        el_output_wrap.hidden = true;
        set_status(
            typeof e === 'string' ? e : 'Failed to render: ' + e.message + '.',
            'error',
        );
    }
}

async function render_file(file) {
    if (!file) {
        return;
    }
    try {
        await render(await file.text());
    } catch (e) {
        set_status('Could not read the file: ' + e.message, 'error');
    }
}

async function fetch_and_render(explicit_url) {
    const url = (explicit_url !== undefined ? explicit_url : el_url_input.value).trim();
    if (!url) {
        set_status('Enter a URL.', 'error');
        return;
    }
    set_status('Fetching...', 'info');
    try {
        await render(await fetch_xml(url), url);
    } catch (e) {
        set_status(
            'Failed to fetch: ' + e.message + '.',
            'error',
        );
    }
}

function dropped_url(data_transfer) {
    const list = data_transfer.getData('text/uri-list');
    if (list) {
        const url = list
            .split('\n')
            .map((line) => line.trim())
            .find((line) => line && !line.startsWith('#'));
        if (url) {
            return url;
        }
    }
    return data_transfer.getData('text/plain').trim();
}

for (const tab of el_tabs) {
    tab.addEventListener('click', () => {
        let name = tab.dataset.panel;
        document.documentElement.dataset.tab = name;
        save('tab', name);
    });
}

el_paste_input.addEventListener('input', () => save('paste', el_paste_input.value));
el_paste_render.addEventListener('click', async () => {
    await render(el_paste_input.value);
});

el_file_input.addEventListener('change', async () => {
    await render_file(el_file_input.files[0]);
    el_file_input.value = '';
});
el_drop_zone.addEventListener('click', () => el_file_input.click());
for (const ev of ['dragenter', 'dragover']) {
    el_drop_zone.addEventListener(ev, (e) => {
        e.preventDefault();
        el_drop_zone.classList.add('dragover');
    });
}
for (const ev of ['dragleave', 'dragend', 'drop']) {
    el_drop_zone.addEventListener(ev, () => {
        el_drop_zone.classList.remove('dragover');
    });
}
el_drop_zone.addEventListener('drop', async (e) => {
    e.preventDefault();
    if (!e.dataTransfer) {
        return;
    }
    const file = e.dataTransfer.files[0];
    if (file) {
        await render_file(file);
        return;
    }
    const url = dropped_url(e.dataTransfer);
    if (url) {
        el_url_input.value = url;
        save('url', url);
        await fetch_and_render(url);
        return;
    }
    set_status('Drop a .xml file or a URL.', 'error');
});

el_url_render.addEventListener('click', () => fetch_and_render());
el_url_input.addEventListener('input', () => save('url', el_url_input.value));
el_url_input.addEventListener('keydown', async (e) => {
    if (e.key === 'Enter') {
        await fetch_and_render();
    }
});
