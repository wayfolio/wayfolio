use {
    crate::{
        render::templates::protocols::toc_description,
        tree::{Protocol, Suite},
    },
    hypertext::prelude::*,
};

pub(crate) fn protocol_toc(inline: bool, suites: &[Suite], ps: &[Protocol]) -> impl Renderable {
    maud! {
        div .protocol_toc {
            @for suite in suites {
                div .toc {
                    div .suite_name { (suite.name) }
                    (protocol_toc_(inline, &suite.protocols))
                }
            }
            (protocol_toc_(inline, ps))
        }
    }
}

fn protocol_toc_(inline: bool, protocols: &[Protocol<'_>]) -> impl Renderable {
    maud! {
        div .toc {
            @for protocol in protocols {
                div .protocol_index {
                    a href={
                        @if inline {
                            "#p-"(protocol.name)
                        } @else {
                            "./p/"(protocol.name)".html"
                        }
                    } { (protocol.name) }
                    (toc_description(&protocol.description))
                }
            }
        }
    }
}
