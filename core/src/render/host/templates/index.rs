use {
    crate::render::{classes::C, templates::wrapper::wrapper},
    hypertext::prelude::*,
};

pub(crate) fn index() -> impl Renderable {
    wrapper(
        false,
        &["common.css", "index.css"],
        maud! {
            meta name="description" content="Wayland protocol documentation";
            title { "Wayfolio" }
        },
        maud! {
            h1 { "Wayfolio" }
            p .(C::index_intro) .(C::lowkey) {
                "HTML documentation for Wayland protocols."
            }
            ul .(C::toc) .(C::index_toc) {
                li {
                    a .(C::generic_link) href="protocols.html" { "Protocol list" }
                    span .(C::index_desc) .(C::lowkey) {
                        "Browse the protocols individually."
                    }
                }
                li {
                    a .(C::generic_link) href="single.html" { "Single page HTML" }
                    span .(C::index_desc) .(C::lowkey) {
                        "One very big document."
                    }
                }
                li {
                    a .(C::generic_link) href="render.html" { "Render your own" }
                    span .(C::index_desc) .(C::lowkey) {
                        "Paste, upload, or link an XML protocol and render it."
                    }
                }
            }
            p .(C::index_outro) .(C::lowkey) {
                "Report issues on "
                a .(C::generic_link) href="https://github.com/wayfolio/wayfolio" { "GitHub" }
                "."
            }
        },
    )
}
