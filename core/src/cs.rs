//! Compositor-support data associated with globals.

#[cfg(feature = "host")]
pub mod parser;

/// A description of a compositor and its capabilities.
///
/// You can create these objects with the [parser].
#[derive(Debug)]
pub struct Compositor<'a> {
    pub(crate) info: CompositorInfo<'a>,
    pub(crate) globals: Vec<Global<'a>>,
}

#[derive(Debug)]
#[doc(hidden)]
pub struct CompositorInfo<'a> {
    pub name: &'a str,
    pub version: &'a str,
}

#[derive(Debug)]
#[doc(hidden)]
pub(crate) struct Global<'a> {
    pub(crate) interface: &'a str,
    pub(crate) version: u32,
}
