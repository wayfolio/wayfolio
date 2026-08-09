use {crate::render::templates::wrapper::wrapper, hypertext::prelude::*};

pub(crate) fn render_url() -> impl Renderable {
    wrapper(
        false,
        &["common.css", "render.css"],
        maud! {
            meta name="description" content="Render a wayland protocol from a URL";
            title {
                "Rendering a wayland protocol…"
            }
            script type="module" src="assets/render_url.js" {}
        },
        maud! {
            div #status .info role="status" aria-live="polite" {
                "Fetching and rendering the protocol…"
            }
            section #error hidden {
                h1 { "Could not render the protocol" }
                p #error_detail {}
                p {
                    "You can go back and try again:"
                }
                ul {
                    li {
                        a #retry_link .generic_link href="render.html" {
                            "Try this URL again on the render page"
                        }
                    }
                    li {
                        a .generic_link href="render.html" {
                            "Go to the render page"
                        }
                    }
                }
            }
        },
    )
}
