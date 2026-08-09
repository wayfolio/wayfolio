use {
    crate::{
        compendium::MemberKind,
        render::templates::protocols::Protocols,
        tree::{Interface, InternalLink, Member, MemberType, Protocol},
    },
    hypertext::{context::AttributeValue, prelude::*},
    isnt::std_1::collections::IsntHashSetExt,
};

impl Protocols<'_> {
    pub(crate) fn internal_link_hue(&self, p: &InternalLink) -> impl Renderable<AttributeValue> {
        let mut hue = "interface_hue";
        if let Some((_, ty)) = p.member {
            hue = match ty {
                MemberKind::Request => "request_hue",
                MemberKind::Event => "event_hue",
                MemberKind::Enum => "enum_hue",
            };
        }
        hue
    }

    pub(crate) fn internal_link(&self, p: &InternalLink) -> impl Renderable<AttributeValue> {
        self.internal_link2(p.protocol, Some(p.interface), p.member)
    }

    pub(crate) fn internal_link2(
        &self,
        p: &str,
        i: Option<&str>,
        m: Option<(&str, MemberKind)>,
    ) -> impl Renderable<AttributeValue> {
        attribute! {
            @if self.local.not_contains(p) {
                "./"
                @if !self.nested {
                    "p/"
                }
                (p) ".html"
            }
            "#"(self.anchor(p, i, m))
        }
    }

    pub(crate) fn protocol_anchor(&self, p: &Protocol) -> impl Renderable<AttributeValue> {
        self.anchor(p.name, None, None)
    }

    pub(crate) fn protocol_link(&self, p: &Protocol) -> impl Renderable<AttributeValue> {
        attribute! {
            "#"(self.protocol_anchor(p))
        }
    }

    pub(crate) fn interface_anchor(
        &self,
        p: &Protocol,
        i: &Interface,
    ) -> impl Renderable<AttributeValue> {
        self.anchor(p.name, Some(i.name), None)
    }

    pub(crate) fn interface_link(
        &self,
        p: &Protocol,
        i: &Interface,
    ) -> impl Renderable<AttributeValue> {
        attribute! {
            "#"(self.interface_anchor(p, i))
        }
    }

    pub(crate) fn member_anchor(
        &self,
        p: &Protocol,
        i: &Interface,
        m: &Member,
    ) -> impl Renderable<AttributeValue> {
        let kind = match &m.ty {
            MemberType::Message(m) if m.is_request => MemberKind::Request,
            MemberType::Message(_) => MemberKind::Event,
            MemberType::Enum(_) => MemberKind::Enum,
        };
        self.anchor(p.name, Some(i.name), Some((m.name, kind)))
    }

    pub(crate) fn member_link(
        &self,
        p: &Protocol,
        i: &Interface,
        m: &Member,
    ) -> impl Renderable<AttributeValue> {
        attribute! {
            "#"(self.member_anchor(p, i, m))
        }
    }

    pub(crate) fn anchor(
        &self,
        p: &str,
        i: Option<&str>,
        m: Option<(&str, MemberKind)>,
    ) -> impl Renderable<AttributeValue> {
        attribute! {
            "p-"
            (p)
            @if let Some(i) = i {
                "-i-" (i)
            }
            @if let Some((m, ty)) = m {
                @let ty = match ty {
                    MemberKind::Request => "re",
                    MemberKind::Event => "ev",
                    MemberKind::Enum => "en",
                };
                "-" (ty) "-" (m)
            }
        }
    }
}
