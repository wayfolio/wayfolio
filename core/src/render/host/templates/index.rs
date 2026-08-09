use {crate::render::templates::wrapper::wrapper, hypertext::prelude::*};

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
            p .index_intro .lowkey {
                "HTML documentation for Wayland protocols."
            }
            ul .toc .index_toc {
                li {
                    a .generic_link href="protocols.html" { "Protocol list" }
                    span .index_desc .lowkey {
                        "Browse the protocols individually."
                    }
                }
                li {
                    a .generic_link href="single.html" { "Single page HTML" }
                    span .index_desc .lowkey {
                        "One very big document."
                    }
                }
                li {
                    a .generic_link href="render.html" { "Render your own" }
                    span .index_desc .lowkey {
                        "Paste, upload, or link an XML protocol and render it."
                    }
                }
            }
            p .index_outro .lowkey {
                "Report issues on "
                a .generic_link href="https://github.com/wayfolio/wayfolio" { "GitHub" }
                "."
            }
        },
    )
}
