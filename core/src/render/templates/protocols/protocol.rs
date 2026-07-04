use {
    crate::{
        render::templates::protocols::{Protocols, toc_description},
        tree::Protocol,
    },
    hypertext::prelude::*,
    isnt::std_1::primitive::IsntSliceExt,
};

impl Protocols<'_> {
    pub(crate) fn protocol(&self, p: &Protocol, include_top: bool) -> impl Renderable {
        let anchor = self.protocol_anchor(p);
        maud! {
            h1 id=(anchor) {
                @if include_top {
                    a .interface-protocol-link href="#top" { "Go to top" }
                    br;
                }
                a href={"#"(anchor)} { (p.name) }
                @if let Some(url) = p.url {
                    " "
                    span .protocol-prefix .protocol-url {
                        a href=(url) { "↗" }
                    }
                }
            }
            @if let Some(c) = &p.copyright {
                details .protocol-prefix .copyright {
                    summary .noselect { "Copyright" }
                    pre { (c.body) }
                }
            }
            @if p.interfaces.is_not_empty() {
                ul .toc {
                    @for i in &p.interfaces {
                        li {
                            span .main-link .interface-hue {
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
