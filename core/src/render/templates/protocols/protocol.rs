use {
    crate::{
        render::{
            classes::C,
            templates::protocols::{Protocols, toc_description},
        },
        tree::Protocol,
    },
    hypertext::prelude::*,
    isnt::std_1::primitive::IsntSliceExt,
};

impl Protocols<'_> {
    pub(crate) fn protocol(&self, p: &Protocol, include_top: bool) -> impl Renderable {
        let anchor = self.protocol_anchor(p);
        let class = include_top.then_some(C::protocol_header);
        maud! {
            h1 .(class) id=(anchor) {
                @if include_top {
                    a .(C::interface_protocol_link) href={"#"(C::top)} { "Go to top" }
                    br;
                }
                a href={"#"(anchor)} { (p.name) }
                @if let Some(url) = p.url {
                    " "
                    span .(C::protocol_prefix) .(C::protocol_url) {
                        a href=(url) { "↗" }
                    }
                }
            }
            @if let Some(c) = &p.copyright {
                details .(C::protocol_prefix) .(C::copyright) {
                    summary .(C::noselect) { "Copyright" }
                    pre { (c.body) }
                }
            }
            @if p.interfaces.is_not_empty() {
                ul .(C::unindented_toc) {
                    @for i in &p.interfaces {
                        li {
                            span .(C::main_link) .(C::interface_hue) {
                                a href={"#"(self.interface_anchor(p, i))} { (i.name) }
                            }
                            (toc_description(&i.description))
                        }
                    }
                }
            }
            @if let Some(d) = &p.description {
                (self.description(d))
            }
            @for i in &p.interfaces {
                (self.interface(p, i))
            }
        }
    }
}
