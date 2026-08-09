use {
    crate::{
        compendium::Compendium,
        render::{
            classes::C,
            templates::{index_link, protocol_toc::protocol_toc, wrapper::wrapper},
        },
        tree::{Description, Protocol, Suite},
    },
    hypertext::{Raw, prelude::*},
    isnt::std_1::primitive::IsntSliceExt,
    std::collections::HashSet,
};

mod description;
mod interface;
mod member;
mod node;
mod protocol;
mod utils;

struct Protocols<'a> {
    c: &'a Compendium<'a>,
    local: HashSet<&'a str>,
    nested: bool,
    embedded_css: bool,
}

impl<'a> Protocols<'a> {
    fn new(
        c: &'a Compendium<'a>,
        nested: bool,
        embedded_css: bool,
        suites: &[Suite<'a>],
        ps: &[Protocol<'a>],
    ) -> Self {
        let local = suites
            .iter()
            .flat_map(|s| &s.protocols)
            .chain(ps)
            .map(|l| l.name)
            .collect();
        Protocols {
            c,
            local,
            nested,
            embedded_css,
        }
    }

    fn protocols(&self, suites: &[Suite], ps: &[Protocol]) -> impl Renderable {
        let include_top = ps.len() > 1
            || suites.len() > 1
            || (ps.is_not_empty() && suites.is_not_empty())
            || suites.iter().any(|s| s.protocols.len() > 1);
        let stylesheets = match self.embedded_css {
            true => &[][..],
            false => &["common.css", "protocol.css"],
        };
        wrapper(
            self.nested,
            stylesheets,
            maud! {
                @if ps.len() == 1 {
                    meta name="description" content={(ps[0].name) " protocol"};
                    title { (ps[0].name) " protocol" }
                } @else {
                    meta name="description" content="Wayland protocols";
                    title { "Wayland protocols" }
                }
                @if self.embedded_css {
                    style {
                        (Raw::dangerously_create(include_str!("../host/statics/common.css")))
                        "\n"
                        (Raw::dangerously_create(include_str!("../host/statics/protocol.css")))
                    }
                }
            },
            maud! {
                div {
                    (index_link(self.nested))
                    @if self.nested {
                        " / "
                        a .(C::generic_link) href="../protocols.html" { "Go to protocols" }
                    }
                }
                @if include_top {
                    (protocol_toc(true, suites, ps))
                }
                @for s in suites {
                    @for p in &s.protocols {
                        (self.protocol(p, include_top))
                    }
                }
                @for p in ps {
                    (self.protocol(p, include_top))
                }
            },
        )
    }
}

pub(crate) fn protocols(
    c: &Compendium<'_>,
    nested: bool,
    embedded_css: bool,
    suites: &[Suite],
    ps: &[Protocol],
) -> impl Renderable {
    maud! {
        (Protocols::new(c, nested, embedded_css, suites, ps).protocols(suites, ps))
    }
}

pub(crate) fn toc_description(v: &Option<Description>) -> impl Renderable {
    maud! {
        @if let Some(v) = v && let Some(v) = v.summary {
            span .(C::toc_summary) { " — " (v) }
        }
    }
}
