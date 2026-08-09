use {
    crate::{
        render::{classes::C, templates::protocols::Protocols},
        tree::Description,
    },
    hypertext::prelude::*,
};

impl Protocols<'_> {
    pub(crate) fn description(&self, v: &Description) -> impl Renderable {
        maud! {
            div .(C::description) {
                @if let Some(v) = v.summary {
                    p .(C::summary) { (v) }
                }
                (self.nodes(&v.children))
            }
        }
    }
}
