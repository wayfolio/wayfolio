import {fetch_xml, generate_html} from './render_common.js';

const el_status = document.getElementById('status');
const el_error = document.getElementById('error');
const el_error_detail = document.getElementById('error_detail');
const el_retry_link = document.getElementById('retry_link');

function fail(message) {
    el_status.hidden = true;
    el_error_detail.textContent = message;
    el_error.hidden = false;
}

async function main() {
    const url = new URLSearchParams(window.location.search).get('url');

    if (!url) {
        fail('No URL was provided. Add a "?url=..." parameter.');
        return;
    }

    el_retry_link.href = 'render.html#url=' + encodeURIComponent(url);

    let xml;
    try {
        xml = await fetch_xml(url);
    } catch (e) {
        fail('Failed to fetch ' + url + ': ' + e.message + '.');
        return;
    }

    try {
        document.documentElement.innerHTML = await generate_html(xml);
    } catch (e) {
        fail(typeof e === 'string' ? e : 'Failed to render the protocol: ' + e.message + '.');
        return;
    }
}

await main();
