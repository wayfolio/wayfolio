use {
    crate::{
        render::{classes::C, templates::protocols::Protocols},
        tree::Node,
    },
    hypertext::prelude::*,
};

impl Protocols<'_> {
    pub(crate) fn nodes(&self, ns: &[Node]) -> impl Renderable {
        maud! {
            @for n in ns {
                (self.node(n))
            }
        }
    }

    pub(crate) fn node(&self, n: &Node) -> impl Renderable {
        maud! {
            @match n {
                Node::Blockquote(v) => {
                    blockquote { (self.nodes(&v.children)) }
                }
                Node::List(v) => {
                    @if v.ordered {
                        ol start=(v.start) {
                            (self.nodes(&v.children))
                        }
                    } @else {
                        ul {
                            (self.nodes(&v.children))
                        }
                    }
                }
                Node::InlineCode(v) => {
                    code { (self.nodes(&v.children)) }
                }
                Node::Delete(v) => {
                    s { (self.nodes(&v.children)) }
                }
                Node::Emphasis(v) => {
                    em { (self.nodes(&v.children)) }
                }
                Node::Link(v) => {
                    a .(C::generic_link) href=(v.url) title=(v.title) { (self.nodes(&v.children)) }
                }
                Node::InternalLink(v) => {
                    span .(C::main_link) .(self.internal_link_hue(v)) {
                        a href=(self.internal_link(v)) { (v.text) }
                    }
                }
                Node::Strong(v) => {
                    strong { (self.nodes(&v.children)) }
                }
                Node::Text(v) => {
                    (v.value)
                }
                Node::Code(v) => {
                    pre { (v.value) }
                }
                Node::ListItem(v) => {
                    li { (self.nodes(&v.children)) }
                }
                Node::Paragraph(v) => {
                    p { (self.nodes(&v.children)) }
                }
            }
        }
    }
}
