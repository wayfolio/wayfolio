use {
    crate::compendium_src::SOURCES,
    compendium::CompendiumError,
    error_reporter::Report,
    thiserror::Error,
    wasm_bindgen::prelude::wasm_bindgen,
    wayfolio::{
        compendium::{self, Compendium},
        ast::parser::{ParserError, parse_xml},
        render::format_wasm_protocols,
    },
};

mod compendium_src;

#[wasm_bindgen]
pub fn generate_html(xml: &str) -> Result<String, String> {
    generate_html_(xml).map_err(|e| Report::new(e).pretty(true).to_string())
}

#[derive(Debug, Error)]
enum Error {
    #[error("Could not parse XML")]
    ParseXml(#[source] ParserError),
    #[error("Could not create compendium")]
    CreateCompendium(#[source] CompendiumError),
}

fn generate_html_(xml: &str) -> Result<String, Error> {
    let protocols = parse_xml(xml.as_bytes()).map_err(Error::ParseXml)?;
    let protocols: Vec<_> = protocols.iter().collect();
    let compendium =
        Compendium::from_sources(SOURCES, &protocols).map_err(Error::CreateCompendium)?;
    Ok(format_wasm_protocols(&compendium, &protocols))
}
