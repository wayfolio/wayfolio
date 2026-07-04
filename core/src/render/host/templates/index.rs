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
            p .index-intro .lowkey {
                "HTML documentation for Wayland protocols."
            }
            ul .toc .index-toc {
                li {
                    a .generic-link href="protocols.html" { "Protocol list" }
                    span .index-desc .lowkey {
                        "Browse the protocols individually."
                    }
                }
                li {
                    a .generic-link href="single.html" { "Single page HTML" }
                    span .index-desc .lowkey {
                        "One very big document."
                    }
                }
                li {
                    a .generic-link href="render.html" { "Render your own" }
                    span .index-desc .lowkey {
                        "Paste, upload, or link an XML protocol and render it."
                    }
                }
            }
            p .index-outro .lowkey {
                "Report issues on "
                a .generic-link href="https://github.com/wayfolio/wayfolio" { "GitHub" }
                "."
            }
        },
    )
}
