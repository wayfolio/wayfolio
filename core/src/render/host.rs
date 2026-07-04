use {
    crate::{
        compendium::Compendium,
        render::{
            format_protocols_,
            host::templates::{
                index::index, protocol_list::protocol_list_page, render::render,
                render_url::render_url,
            },
        },
        tree::{Protocol, Suite},
    },
    hypertext::prelude::*,
};

pub(crate) mod statics;
pub(crate) mod templates;

pub(crate) fn format_protocol_list(suites: &[Suite], protocols: &[Protocol<'_>]) -> String {
    protocol_list_page(suites, protocols).render().into_inner()
}

pub(crate) fn format_index() -> String {
    index().render().into_inner()
}

pub(crate) fn format_render() -> String {
    render().render().into_inner()
}

pub(crate) fn format_render_url() -> String {
    render_url().render().into_inner()
}

pub(crate) fn format_protocols(
    compendium: &Compendium<'_>,
    nested: bool,
    suites: &[Suite<'_>],
    protocols: &[Protocol<'_>],
) -> String {
    format_protocols_(compendium, nested, false, suites, protocols)
}
