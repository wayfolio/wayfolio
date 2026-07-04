//! Generation of a site on the filesystem.

use {
    crate::{
        analysis::analyze,
        ast,
        compendium::Compendium,
        render::host::{
            format_index, format_protocol_list, format_protocols, format_render, format_render_url,
            statics::{self, Static},
        },
        tree::{Protocol, Suite},
    },
    std::{
        fs,
        path::{Path, PathBuf},
        slice,
    },
    thiserror::Error,
};

/// An error that can occur while generating a site with [`generate_site`].
#[derive(Debug, Error)]
pub enum SiteError {
    #[error("Protocol name {0} doesn't match [a-zA-Z0-9_-]*")]
    MalformedProtocolName(String),
    #[error("Could not create directory")]
    CreateDir(#[source] std::io::Error),
    #[error("Could not write {0}")]
    WriteFile(PathBuf, #[source] std::io::Error),
    #[error("Could not write protocol file")]
    WriteProtocol(#[source] std::io::Error),
}

/// Generates a site.
///
/// The directory `dir` is created if necessary. Existing files with the same names are
/// overwritten.
///
/// The generated site does not contain the WASM module. See the readme of the WASM module
/// for how to create it.
pub fn generate_site(
    dir: impl AsRef<Path>,
    compendium: &Compendium,
    suites: &[&ast::Suite],
    protocols: &[&ast::Protocol],
) -> Result<(), SiteError> {
    generate_site_(dir.as_ref(), compendium, suites, protocols)
}

fn generate_site_(
    dir: &Path,
    compendium: &Compendium,
    suites: &[&ast::Suite],
    protocols: &[&ast::Protocol],
) -> Result<(), SiteError> {
    let assets_dir = dir.join("assets");
    let p_dir = dir.join("p");

    fs::create_dir_all(dir).map_err(SiteError::CreateDir)?;
    fs::create_dir_all(&assets_dir).map_err(SiteError::CreateDir)?;
    fs::create_dir_all(&p_dir).map_err(SiteError::CreateDir)?;

    let (suites, protocols) = analyze(compendium, suites, protocols);

    let write_file = |path: &Path, contents: &str| {
        fs::write(path, contents).map_err(|e| SiteError::WriteFile(path.to_path_buf(), e))
    };

    let write_root = |name: &str, contents: &str| write_file(&dir.join(name), contents);
    write_root("index.html", &format_index())?;
    write_root("render.html", &format_render())?;
    write_root("render-url.html", &format_render_url())?;
    write_root("protocols.html", &format_protocol_list(&suites, &protocols))?;
    write_root(
        "single.html",
        &format_protocols(compendium, false, &suites, &protocols),
    )?;

    let write_asset = |s: &Static| write_file(&assets_dir.join(s.name), s.body);
    write_asset(&statics::COMMON_CSS)?;
    write_asset(&statics::FAVICON_SVG)?;
    write_asset(&statics::INDEX_CSS)?;
    write_asset(&statics::PROTOCOL_CSS)?;
    write_asset(&statics::RENDER_CSS)?;
    write_asset(&statics::RENDER_JS)?;
    write_asset(&statics::RENDER_COMMON_JS)?;
    write_asset(&statics::RENDER_URL_JS)?;

    for p in all_protocols(&suites, &protocols) {
        for c in p.name.chars() {
            if !matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-') {
                return Err(SiteError::MalformedProtocolName(p.name.to_string()));
            }
        }
        fs::write(
            p_dir.join(p.name).with_added_extension("html"),
            format_protocols(compendium, true, &[], slice::from_ref(p)),
        )
        .map_err(SiteError::WriteProtocol)?;
    }

    Ok(())
}

fn all_protocols<'a>(
    suites: &'a [Suite<'a>],
    protocols: &'a [Protocol<'a>],
) -> impl Iterator<Item = &'a Protocol<'a>> {
    suites.iter().flat_map(|s| &s.protocols).chain(protocols)
}
