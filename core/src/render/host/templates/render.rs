use {
    crate::render::templates::{index_link, wrapper::wrapper},
    hypertext::{Raw, prelude::*},
};

const PLACEHOLDER: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<protocol name="my_protocol">
  ...
</protocol>"#;

pub(crate) fn render() -> impl Renderable {
    wrapper(
        false,
        &["common.css", "render.css"],
        maud! {
            meta name="description" content="Render a Wayland protocol";
            title {
                "Render a Wayland protocol"
            }
            script {
                // this must run synchronously so that the first frame is perfect
                (Raw::dangerously_create(include_str!("render_init.js")))
            }
            script type="module" src="assets/render.js" {}
        },
        maud! {
            div {
                (index_link(false))
            }
            h1 { "Render a Wayland protocol" }
            p .render-intro {
                "Provide a protocol XML file and it will be rendered as HTML right "
                "here in your browser."
            }
            div .tabs {
                button .tab type="button" data-panel="paste" {
                    "Paste XML"
                }
                button .tab type="button" data-panel="file" {
                    "Upload a file"
                }
                button .tab type="button" data-panel="url" {
                    "From a URL"
                }
            }
            section .panel data-panel="paste" {
                label .field-label for="paste-input" { "Paste the contents of your .xml file:" }
                textarea #paste-input rows="16" spellcheck="false" placeholder=(PLACEHOLDER) { }
                div .field-row {
                    button #paste-render .render-btn type="button" { "Render" }
                }
            }
            section .panel data-panel="file" {
                button #drop-zone type="button" {
                    div .drop-zone-title { "Drop an .xml file or a URL here" }
                    div .drop-zone-hint { "or click to choose a file from your computer." }
                }
                input #file-input type="file" accept=".xml,text/xml,application/xml" hidden;
            }
            section .panel data-panel="url" {
                label .field-label for="url-input" { "Enter the URL of an .xml file:" }
                div .field-row {
                    input #url-input type="url" inputmode="url" placeholder="https://example.com/my-protocol.xml";
                    button #url-render .render-btn type="button" { "Fetch & render" }
                }
                p .field-hint {
                    "The file is fetched from your browser, so the server must allow "
                    "cross-origin requests."
                }
            }
            div #status {}
            section #output-wrap hidden {
                div .output-bar {
                    h2 .output-title { "Result" }
                    span .output-links {
                        a #share-link .generic-link hidden {
                            "Shareable link"
                        }
                        a #open-link .generic-link target="_blank" rel="noopener" {
                            "Open in a new tab"
                        }
                    }
                }
                iframe #output title="Rendered protocol" {}
            }
        },
    )
}
