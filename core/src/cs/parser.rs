//! Parser for the compositor-support database.

use {
    crate::cs::{Compositor, CompositorInfo, Global},
    serde::Deserialize,
    std::{io, path::Path},
    thiserror::Error,
};

/// An error that can occur while parsing compositor-support data.
#[derive(Debug, Error)]
pub enum CompositorSupportError {
    #[error("read_dir failed")]
    ReadDir(#[source] io::Error),
    #[error("Could not read file")]
    ReadFile(#[source] io::Error),
    #[error("Could not deserialize json")]
    Deserialize(#[source] serde_json::Error),
    #[error("File name does not end in .json")]
    FileNameJson,
    #[error("File name is not UTF-8")]
    FileName,
}

/// Parsed compositor-support data.
///
/// Use [`get`](Self::get) to get the vector that can be passed to the compendium.
pub struct CompositorSupport {
    compositors: Vec<JsonNamedCompositor>,
}

struct JsonNamedCompositor {
    name: String,
    compositor: JsonCompositor,
}

#[derive(Deserialize)]
struct JsonCompositor {
    version: String,
    globals: Vec<JsonGlobal>,
}

#[derive(Deserialize)]
struct JsonGlobal {
    interface: String,
    version: u32,
}

impl CompositorSupport {
    /// Parses the compositor-support database.
    ///
    /// `dir` should point to a checkout of the [compositor-support] repository.
    ///
    /// [compositor-support]: https://github.com/wayfolio/compositor-support
    pub fn parse(dir: impl AsRef<Path>) -> Result<Self, CompositorSupportError> {
        Self::parse_(dir.as_ref())
    }

    fn parse_(dir: &Path) -> Result<Self, CompositorSupportError> {
        let data = dir.join("data");
        let mut res = vec![];
        for file in data.read_dir().map_err(CompositorSupportError::ReadDir)? {
            let file = file.map_err(CompositorSupportError::ReadDir)?.path();
            let data = std::fs::read_to_string(&file).map_err(CompositorSupportError::ReadFile)?;
            let compositor: JsonCompositor =
                serde_json::from_str(&data).map_err(CompositorSupportError::Deserialize)?;
            let name = file
                .file_stem()
                .ok_or(CompositorSupportError::FileNameJson)?
                .to_str()
                .ok_or(CompositorSupportError::FileName)?
                .to_string();
            res.push(JsonNamedCompositor { name, compositor });
        }
        Ok(CompositorSupport { compositors: res })
    }

    /// Borrows the compositor-support data to pass to the compendium.
    pub fn get(&self) -> Vec<Compositor<'_>> {
        self.compositors
            .iter()
            .map(|c| Compositor {
                info: CompositorInfo {
                    name: &c.name,
                    version: &c.compositor.version,
                },
                globals: c
                    .compositor
                    .globals
                    .iter()
                    .map(|g| Global {
                        interface: &g.interface,
                        version: g.version,
                    })
                    .collect(),
            })
            .collect()
    }
}
