use {
    crate::{
        render::templates::{index_link, protocol_toc::protocol_toc, wrapper::wrapper},
        tree::{Protocol, Suite},
    },
    hypertext::prelude::*,
};

pub(crate) fn protocol_list_page(suites: &[Suite], ps: &[Protocol]) -> impl Renderable {
    wrapper(
        false,
        &["common.css"],
        maud! {
            meta name="description" content="Wayland protocols";
            title {
                "Wayland protocols"
            }
        },
        maud! {
            div {
                (index_link(false))
            }
            (protocol_toc(false, suites, ps))
        },
    )
}
