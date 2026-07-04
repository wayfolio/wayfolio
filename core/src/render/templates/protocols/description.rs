use {
    crate::{render::templates::protocols::Protocols, tree::Description},
    hypertext::prelude::*,
};

impl Protocols<'_> {
    pub(crate) fn description(&self, v: &Description) -> impl Renderable {
        maud! {
            div .description {
                @if let Some(v) = v.summary {
                    p .summary { (v) }
                }
                (self.nodes(&v.children))
            }
        }
    }
}
