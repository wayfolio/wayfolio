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
            p .render_intro {
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
                label .field_label for="paste_input" { "Paste the contents of your .xml file:" }
                textarea #paste_input rows="16" spellcheck="false" placeholder=(PLACEHOLDER) { }
                div .field_row {
                    button #paste_render .render_btn type="button" { "Render" }
                }
            }
            section .panel data-panel="file" {
                button #drop_zone type="button" {
                    div .drop_zone_title { "Drop an .xml file or a URL here" }
                    div .drop_zone_hint { "or click to choose a file from your computer." }
                }
                input #file_input type="file" accept=".xml,text/xml,application/xml" hidden;
            }
            section .panel data-panel="url" {
                label .field_label for="url_input" { "Enter the URL of an .xml file:" }
                div .field_row {
                    input #url_input type="url" inputmode="url" placeholder="https://example.com/my-protocol.xml";
                    button #url_render .render_btn type="button" { "Fetch & render" }
                }
                p .field_hint {
                    "The file is fetched from your browser, so the server must allow "
                    "cross-origin requests."
                }
            }
            div #status {}
            section #output_wrap hidden {
                div .output_bar {
                    h2 .output_title { "Result" }
                    span .output_links {
                        a #share_link .generic_link hidden {
                            "Shareable link"
                        }
                        a #open_link .generic_link target="_blank" rel="noopener" {
                            "Open in a new tab"
                        }
                    }
                }
                iframe #output title="Rendered protocol" {}
            }
        },
    )
}
