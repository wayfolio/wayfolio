//! Generate static HTML documentation from Wayland protocol definitions.
//!
//! # Example
//!
//! ```
//! use {
//!     std::fs,
//!     wayfolio::{
//!         ast::parser::parse_xml, compendium::Compendium, cs::parser::CompositorSupport,
//!         site::generate_site,
//!     },
//! };
//!
//! # fn f() {
//! // Load data from the compositor-support database. This is optional.
//! let support = CompositorSupport::parse("/path/to/compositor-support").unwrap();
//! let support = support.get();
//!
//! // Load the protocols.
//! let mut protocols = vec![];
//! for file in fs::read_dir("/path/to/xml/files").unwrap() {
//!     let file = file.unwrap();
//!     let xml = fs::read(&file.path()).unwrap();
//!     let protocol = parse_xml(&xml).unwrap();
//!     protocols.extend(protocol);
//! }
//! let protocols: Vec<_> = protocols.iter().collect();
//!
//! // Create the compendium.
//! let compendium = Compendium::new(&protocols, &support).unwrap();
//!
//! // Generate most files for the site.
//! generate_site("page", &compendium, &[], &protocols).unwrap();
//!
//! // Generate the compendium_src.rs file for the WASM module.
//! // The WASM module must be compiled and copied to the `page` directory separately.
//! // See the readme of the WASM module.
//! fs::write(
//!     "/path/to/wayfolio-wasm/src/compendium_src.rs",
//!     compendium.create_src(),
//! )
//! .unwrap();
//! # }
//! ```

pub use wayfolio_core::*;
