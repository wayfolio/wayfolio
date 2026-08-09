use {
    crate::render::{
        classes::C,
        templates::{index_link, wrapper::wrapper},
    },
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
            p .(C::render_intro) {
                "Provide a protocol XML file and it will be rendered as HTML right "
                "here in your browser."
            }
            div .(C::tabs) {
                button .(C::tab) type="button" data-panel="paste" {
                    "Paste XML"
                }
                button .(C::tab) type="button" data-panel="file" {
                    "Upload a file"
                }
                button .(C::tab) type="button" data-panel="url" {
                    "From a URL"
                }
            }
            section .(C::panel) data-panel="paste" {
                label .(C::field_label) for=(C::paste_input) { "Paste the contents of your .xml file:" }
                textarea #(C::paste_input) rows="16" spellcheck="false" placeholder=(PLACEHOLDER) { }
                div .(C::field_row) {
                    button #(C::paste_render) .(C::render_btn) type="button" { "Render" }
                }
            }
            section .(C::panel) data-panel="file" {
                button #(C::drop_zone) type="button" {
                    div .(C::drop_zone_title) { "Drop an .xml file or a URL here" }
                    div .(C::drop_zone_hint) { "or click to choose a file from your computer." }
                }
                input #(C::file_input) type="file" accept=".xml,text/xml,application/xml" hidden;
            }
            section .(C::panel) data-panel="url" {
                label .(C::field_label) for=(C::url_input) { "Enter the URL of an .xml file:" }
                div .(C::field_row) {
                    input #(C::url_input) type="url" inputmode="url" placeholder="https://example.com/my-protocol.xml";
                    button #(C::url_render) .(C::render_btn) type="button" { "Fetch & render" }
                }
                p .(C::field_hint) {
                    "The file is fetched from your browser, so the server must allow "
                    "cross-origin requests."
                }
            }
            div #(C::status) {}
            section #(C::output_wrap) hidden {
                div .(C::output_bar) {
                    h2 .(C::output_title) { "Result" }
                    span .(C::output_links) {
                        a #(C::share_link) .(C::generic_link) hidden {
                            "Shareable link"
                        }
                        a #(C::open_link) .(C::generic_link) target="_blank" rel="noopener" {
                            "Open in a new tab"
                        }
                    }
                }
                iframe #(C::output) title="Rendered protocol" {}
            }
        },
    )
}
