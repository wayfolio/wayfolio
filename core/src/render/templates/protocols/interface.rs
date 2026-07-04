use {
    crate::{
        compendium::Compositor,
        render::templates::protocols::{Protocols, toc_description},
        tree::{Interface, MemberType, Protocol},
    },
    hypertext::prelude::*,
    isnt::std_1::primitive::IsntSliceExt,
    std::collections::HashMap,
};

impl Protocols<'_> {
    pub(crate) fn interface(&self, p: &Protocol, i: &Interface) -> impl Renderable {
        let support = self.c.get_compositor_support(i.name);
        let mut sp = HashMap::new();
        for s in support {
            sp.entry(s.version).or_insert_with(Vec::new).push(s);
        }
        let mut sp: Vec<_> = sp.into_iter().collect();
        sp.sort_by_key(|s| s.0.wrapping_neg());
        maud! {
            h2 id=(self.interface_anchor(p, i)) {
                a .interface-protocol-link href=(self.protocol_link(p)) { (p.name) }
                br;
                span .interface-name .interface-hue {
                    a href=(self.interface_link(p, i)) { (i.name) }
                }
            }
            div .badges {
                @if i.frozen == Some(true) {
                    span .badge-normal .frozen-hue { "This interface is frozen" }
                } @else {
                    span .badge-normal .interface-hue { "Version " (i.version) }
                }
            }
            @if i.members.is_not_empty() {
                ul .toc {
                    @for m in &i.members {
                        li {
                            span .main-link .(
                                match &m.ty {
                                    MemberType::Message(v) if v.is_request => "request-hue",
                                    MemberType::Message(_) => "event-hue",
                                    MemberType::Enum(_) => "enum-hue",
                                }
                            ) {
                                a href=(self.member_link(p, i, m)) { (m.name) }
                            }
                            (toc_description(&m.description))
                        }
                    }
                }
            }
            @if support.is_not_empty() {
                (compositor_support_boxes(&sp))
            }
            @if let Some(d) = &i.description {
                (self.description(d))
            }
            @for m in &i.members {
                (self.member(p, i, m))
            }
        }
    }
}

fn compositor_support_boxes(sp: &[(u32, Vec<&Compositor>)]) -> impl Renderable {
    maud! {
        div .compositor-support-title { "Compositor Support" }
        div .compositor-support {
            @for c in sp {
                (compositor_support_box(c.0, &c.1))
            }
        }
    }
}

fn compositor_support_box(version: u32, compositors: &[&Compositor]) -> impl Renderable {
    let cols = ((compositors.len() as f64).sqrt().floor() as usize).max(1);
    let colw = compositors
        .iter()
        .map(|e| e.info.name.len() + 1 + (e.info.version.len() as f64 * 0.85).ceil() as usize)
        .max()
        .unwrap_or(1);
    let style = attribute! {
        "--cols:"(cols)";--colw:"(colw)"ch"
    };
    maud! {
        div .compositor-support-version {
            span .badge-normal .interface-hue { "Version " (version) }
            div .compositor-support-version-list style=(style) {
                @for c in compositors {
                    div .compositor-support-version-list-element {
                        (c.info.name)
                        " "
                        span .compositor-version { (c.info.version) }
                    }
                }
            }
        }
    }
}
