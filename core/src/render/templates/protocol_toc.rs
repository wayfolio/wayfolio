use {
    crate::{
        render::{classes::C, templates::protocols::toc_description},
        tree::{Protocol, Suite},
    },
    hypertext::prelude::*,
};

pub(crate) fn protocol_toc(inline: bool, suites: &[Suite], ps: &[Protocol]) -> impl Renderable {
    maud! {
        div .(C::protocol_toc) {
            @for suite in suites {
                div .(C::toc) {
                    div .(C::suite_name) { (suite.name) }
                    (protocol_toc_(inline, &suite.protocols))
                }
            }
            (protocol_toc_(inline, ps))
        }
    }
}

fn protocol_toc_(inline: bool, protocols: &[Protocol<'_>]) -> impl Renderable {
    maud! {
        div .(C::toc) {
            @for protocol in protocols {
                div .(C::protocol_index) {
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
