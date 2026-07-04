use {
    crate::{
        analysis::analyze,
        ast,
        compendium::Compendium,
        tree::{Protocol, Suite},
    },
    hypertext::Renderable,
};

#[cfg(feature = "host")]
pub(crate) mod host;
mod templates;

#[doc(hidden)]
pub fn format_wasm_protocols(compendium: &Compendium<'_>, protocols: &[&ast::Protocol]) -> String {
    let (_, protocols) = analyze(compendium, &[], protocols);
    format_protocols_(compendium, false, true, &[], &protocols)
}

fn format_protocols_(
    compendium: &Compendium<'_>,
    nested: bool,
    embedded_css: bool,
    suites: &[Suite<'_>],
    protocols: &[Protocol<'_>],
) -> String {
    templates::protocols::protocols(compendium, nested, embedded_css, suites, protocols)
        .render()
        .into_inner()
}
