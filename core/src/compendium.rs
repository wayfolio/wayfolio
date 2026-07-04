//! The index of known interfaces, members, and compositor support.

use {
    crate::{
        ast::{self, MemberType},
        cs::{self, CompositorInfo},
    },
    aho_corasick::{AhoCorasick, MatchKind},
    linearize::{Linearize, StaticMap},
    std::{
        cmp,
        collections::{HashMap, hash_map::Entry},
    },
    thiserror::Error,
};

#[cfg(feature = "host")]
mod host;

/// An error that can occur while building a [`Compendium`].
#[derive(Debug, Error)]
pub enum CompendiumError {
    #[error("Could not create text matcher")]
    Matcher(#[source] aho_corasick::BuildError),
}

/// The index of known interfaces, members, and compositor support.
#[derive(Debug)]
pub struct Compendium<'a> {
    interfaces: HashMap<&'a str, Interface<'a>>,
    matcher: Matcher,
}

#[derive(Debug)]
struct Interface<'a> {
    protocol: &'a str,
    members: Members<'a>,
    compositors: Vec<Compositor<'a>>,
}

type Members<'a> = HashMap<&'a str, StaticMap<MemberKind, bool>>;

#[derive(Debug)]
struct Matcher {
    matcher: AhoCorasick,
    interfaces_lo: usize,
}

pub(crate) struct Reference<'a> {
    pub(crate) offset: usize,
    pub(crate) len: usize,
    pub(crate) protocol: &'a str,
    pub(crate) interface: &'a str,
    pub(crate) member: Option<(&'a str, MemberKind)>,
}

#[doc(hidden)]
#[derive(Copy, Clone, Debug)]
pub struct Compositor<'a> {
    pub info: &'a CompositorInfo<'a>,
    pub version: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Linearize)]
pub(crate) enum MemberKind {
    Request,
    Event,
    Enum,
}

#[doc(hidden)]
pub struct SourceInterface<'a> {
    pub name: &'a str,
    pub protocol: &'a str,
    pub members: &'a [(&'a str, [bool; 3])],
    pub compositors: &'a [Compositor<'a>],
}

fn add_interfaces<'a>(
    interfaces: &mut HashMap<&'a str, Interface<'a>>,
    protocols: &[&'a ast::Protocol],
) {
    for protocol in protocols.iter().rev() {
        for interface in &protocol.interfaces {
            let mut members = Members::with_capacity(interface.members.len());
            for v in &interface.members {
                let ty = match &v.ty {
                    MemberType::Message(v) if v.is_request => MemberKind::Request,
                    MemberType::Message(_) => MemberKind::Event,
                    MemberType::Enum(_) => MemberKind::Enum,
                };
                members.entry(v.name.as_str()).or_default()[ty] = true;
            }
            match interfaces.entry(&*interface.name) {
                Entry::Occupied(v) => {
                    let v = v.into_mut();
                    v.protocol = &protocol.name;
                    v.members = members;
                }
                Entry::Vacant(v) => {
                    v.insert(Interface {
                        protocol: &protocol.name,
                        members,
                        compositors: Default::default(),
                    });
                }
            }
        }
    }
}

impl<'a> Compendium<'a> {
    /// Builds a compendium from protocols and compositor-support data.
    ///
    /// `protocols` should contain the protocols in descending order of precedence. This
    /// only matters if multiple protocols define the same interface, in which case the
    /// compendium will resolve references to the first protocol from the list.
    pub fn new(
        protocols: &[&'a ast::Protocol],
        compositors: &'a [cs::Compositor<'a>],
    ) -> Result<Self, CompendiumError> {
        let mut interfaces = HashMap::new();
        add_interfaces(&mut interfaces, protocols);
        for c in compositors {
            for g in &c.globals {
                if let Some(i) = interfaces.get_mut(g.interface) {
                    i.compositors.push(Compositor {
                        info: &c.info,
                        version: g.version,
                    });
                }
            }
        }
        for i in interfaces.values_mut() {
            i.compositors.sort_by_key(|c| c.info.name);
        }
        Ok(Compendium {
            matcher: create_matcher(&interfaces)?,
            interfaces,
        })
    }

    pub(crate) fn find_protocol(
        &self,
        interface: &str,
        member: Option<(&str, MemberKind)>,
    ) -> Option<&str> {
        let def = self.interfaces.get(interface)?;
        if let Some((member, ty)) = member
            && !def.members.get(member)?[ty]
        {
            return None;
        }
        Some(def.protocol)
    }

    pub(crate) fn find_text_reference(&self, haystack: &str) -> Option<Reference<'_>> {
        let res = self.matcher.matcher.find(haystack)?;
        let needle = &haystack[res.range()];
        let pattern = res.pattern().as_usize();
        if pattern >= self.matcher.interfaces_lo {
            let (interface, def) = self.interfaces.get_key_value(needle)?;
            Some(Reference {
                offset: res.start(),
                len: res.len(),
                protocol: def.protocol,
                interface,
                member: None,
            })
        } else {
            let (interface, member) = needle.split_once(".")?;
            let (interface, def) = self.interfaces.get_key_value(interface)?;
            let (member, tys) = def.members.get_key_value(member)?;
            let ty = tys
                .iter()
                .find_map(|(ty, valid)| valid.then_some(ty))
                .unwrap_or(MemberKind::Request);
            Some(Reference {
                offset: res.start(),
                len: res.len(),
                protocol: def.protocol,
                interface,
                member: Some((member, ty)),
            })
        }
    }

    pub(crate) fn get_compositor_support(&self, interface: &str) -> &[Compositor<'a>] {
        match self.interfaces.get(interface) {
            None => &[],
            Some(i) => &i.compositors,
        }
    }

    #[doc(hidden)]
    pub fn from_sources(
        sources: &[SourceInterface<'a>],
        overrides: &[&'a ast::Protocol],
    ) -> Result<Self, CompendiumError> {
        let mut interfaces = HashMap::with_capacity(sources.len() + overrides.len());
        for source in sources {
            let mut members = Members::with_capacity(source.members.len());
            for &(name, data) in source.members {
                members.insert(name, StaticMap(data));
            }
            interfaces.insert(
                source.name,
                Interface {
                    protocol: source.protocol,
                    members,
                    compositors: source.compositors.to_vec(),
                },
            );
        }
        add_interfaces(&mut interfaces, overrides);
        Ok(Self {
            matcher: create_matcher(&interfaces)?,
            interfaces,
        })
    }
}

fn create_matcher(interfaces: &HashMap<&str, Interface<'_>>) -> Result<Matcher, CompendiumError> {
    let mut patterns = vec![];
    for (name, interface) in interfaces {
        for v in interface.members.keys() {
            patterns.push(format!("{name}.{v}"));
        }
    }
    patterns.sort_by_key(|p| cmp::Reverse(p.len()));
    let interfaces_lo = patterns.len();
    for name in interfaces.keys() {
        patterns.push(name.to_string());
    }
    patterns[interfaces_lo..].sort_by_key(|p| cmp::Reverse(p.len()));
    let matcher = AhoCorasick::builder()
        .match_kind(MatchKind::LeftmostFirst)
        .build(patterns)
        .map_err(CompendiumError::Matcher)?;
    Ok(Matcher {
        matcher,
        interfaces_lo,
    })
}
