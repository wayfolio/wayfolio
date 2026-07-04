import {default as init_wasm, generate_html as real_generate_html} from '../wasm.js';

init_wasm();

async function gitlab_graphql(origin, query, variables) {
    const response = await fetch(origin + '/api/graphql', {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: JSON.stringify({query, variables}),
    });
    if (!response.ok) {
        throw new Error('HTTP ' + response.status + ' ' + response.statusText);
    }
    const body = await response.json();
    if (body.errors && body.errors.length) {
        throw new Error(body.errors[0].message);
    }
    return body;
}

async function handle_gitlab_mr(url) {
    const match = url.pathname.match(/^\/(.+?)\/-\/merge_requests\/(\d+)\/?/);
    if (!match) {
        return null;
    }
    let origin = url.origin;
    let project = decodeURIComponent(match[1]);
    let iid = match[2];
    const mr = await gitlab_graphql(
        origin,
        'query($project:ID!,$iid:String!){project(fullPath:$project){' +
        'mergeRequest(iid:$iid){diffHeadSha diffStats{path}}}}',
        {project, iid},
    );
    const merge = mr?.data?.project?.mergeRequest;
    if (!merge) {
        throw new Error("merge request not found (is the project public?)");
    }
    const paths = (merge.diffStats || [])
        .map((s) => s.path)
        .filter((p) => p.endsWith('.xml'));
    if (!paths.length) {
        throw new Error('the merge request contains no .xml files');
    }
    const blobs = await gitlab_graphql(
        origin,
        'query($project:ID!,$paths:[String!]!,$ref:String){project(fullPath:$project){' +
        'repository{blobs(paths:$paths,ref:$ref){nodes{rawTextBlob}}}}}',
        {project, paths: paths, ref: merge.diffHeadSha},
    );
    const texts = (blobs?.data?.project?.repository?.blobs?.nodes || [])
        .map((n) => n.rawTextBlob)
        .filter((t) => t != null);
    if (!texts.length) {
        throw new Error('could not read the changed files');
    }
    return texts.join('\n');
}

async function handle_gitlab_blob(url) {
    const match = url.pathname.match(/^\/(.+?)\/-\/(?:raw|blob)\/(.+)$/);
    if (!match) {
        return null;
    }
    const segments = match[2].split('/').map(decodeURIComponent);
    if (segments.length < 2) {
        return null;
    }
    let origin = url.origin;
    let project = decodeURIComponent(match[1]);
    const candidates = [];
    for (let k = segments.length - 1; k >= 1; k--) {
        candidates.push({ref: segments.slice(0, k).join('/'), path: segments.slice(k).join('/')});
    }
    const params = ['$project:ID!']
        .concat(candidates.map((_, i) => `$p${i}:[String!]!,$r${i}:String`))
        .join(',');
    const fields = candidates
        .map((_, i) => `c${i}:blobs(paths:$p${i},ref:$r${i}){nodes{rawTextBlob}}`)
        .join(' ');
    const query = `query(${params}){project(fullPath:$project){repository{${fields}}}}`;
    const variables = {project};
    candidates.forEach((c, i) => {
        variables['p' + i] = [c.path];
        variables['r' + i] = c.ref;
    });
    const body = await gitlab_graphql(origin, query, variables);
    const repo = body?.data?.project?.repository;
    if (repo) {
        for (let i = 0; i < candidates.length; i++) {
            const text = repo['c' + i]?.nodes?.[0]?.rawTextBlob;
            if (text != null) {
                return text;
            }
        }
    }
    throw new Error('no such file (is the project public?)');
}

function parse_gitlab_url(url) {
    let parsed;
    try {
        parsed = new URL(url);
    } catch (e) {
        return null;
    }
    if (!parsed.hostname.startsWith('gitlab.')) {
        return null;
    }
    return parsed;
}

export async function fetch_xml(url) {
    let gitlab_url = parse_gitlab_url(url);
    if (gitlab_url) {
        let blob;
        blob = await handle_gitlab_mr(gitlab_url);
        if (blob) {
            return blob;
        }
        blob = await handle_gitlab_blob(gitlab_url);
        if (blob) {
            return blob;
        }
    }
    const response = await fetch(url);
    if (!response.ok) {
        throw new Error('HTTP ' + response.status + ' ' + response.statusText);
    }
    return await response.text();
}

export async function generate_html(xml) {
    await init_wasm();
    return real_generate_html(xml);
}
