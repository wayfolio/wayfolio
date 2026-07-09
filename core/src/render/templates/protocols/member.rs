use {
    crate::{
        ast::{ArgType, MessageType},
        compendium::MemberKind,
        render::templates::protocols::Protocols,
        tree::{
            Arg, ArgEnum, ArgInterface, Description, Enum, Interface, Member, MemberType, Message,
            Protocol,
        },
    },
    hypertext::prelude::*,
    isnt::std_1::primitive::IsntSliceExt,
};

impl Protocols<'_> {
    pub(crate) fn member(&self, p: &Protocol, i: &Interface, m: &Member) -> impl Renderable {
        maud! {
            @match &m.ty {
                MemberType::Message(v) => (self.message(p, i, m, v)),
                MemberType::Enum(v) => (self.enum_(p, i, m, v)),
            }
        }
    }

    fn message(&self, p: &Protocol, i: &Interface, m: &Member, v: &Message) -> impl Renderable {
        maud! {
            (self.message_signature(p, i, m, v))
            div .message-body {
                div .badges {
                    @if v.ty == Some(MessageType::Destructor) {
                        span .badge-destructor { "Destructor" }
                    }
                    (self.since_badge(m.since))
                    (self.deprecated_since_badge(m.deprecated_since))
                }
                ul {
                    @for a in &v.args {
                        (self.arg_description(a.name, a.summary, &a.description, None, None))
                    }
                }
                @if let Some(v) = &m.description {
                    (self.description(v))
                }
            }
        }
    }

    fn message_signature(
        &self,
        p: &Protocol,
        i: &Interface,
        m: &Member,
        v: &Message,
    ) -> impl Renderable {
        let hue = match v.is_request {
            true => "request-hue",
            false => "event-hue",
        };
        let keyword = match v.is_request {
            true => "request",
            false => "event",
        };
        let paren = match v.is_request {
            true => "request-paren",
            false => "event-paren",
        };
        maud! {
            pre .message-signature id=(self.member_anchor(p, i, m)) {
                span .message-interface-link {
                    a href=(self.interface_link(p, i)) {
                        (i.name)
                    }
                }
                "\n"
                span .keyword .(hue) { (keyword) }
                " "
                span .member-name .(hue) {
                    a href=(self.member_link(p, i, m)) {
                        (m.name)
                    }
                }
                span .(paren) { "(" }
                @if v.args.is_not_empty() {
                    "\n"
                }
                @for a in &v.args {
                    "    "
                    span .arg-name { (a.name) }
                    span .arg-colon { ":" }
                    " "
                    (self.arg_type(i, a))
                    span .arg-comma { "," }
                    "\n"
                }
                span .(paren) { ")" }
            }
        }
    }

    fn wrap_in_link(&self, a: &Arg, r: impl Renderable) -> impl Renderable {
        let mut protocol = None;
        let mut interface = None;
        let mut member = None;
        if let Some(i) = &a.interface
            && let ArgInterface::Known(p, i) = i
        {
            protocol = Some(*p);
            interface = Some(*i);
        } else if let Some(v) = &a.enum_
            && let ArgEnum::Known(p, i, e) = v
        {
            protocol = Some(*p);
            interface = Some(*i);
            member = Some((*e, MemberKind::Enum));
        }
        let link = protocol.map(|p| self.internal_link2(p, interface, member));
        maud! {
            @if let Some(link) = &link {
                span .arg-link {
                    a href=(link) {
                        (r)
                    }
                }
            } @else {
                (r)
            }
        }
    }

    fn arg_type(&self, interface: &Interface, a: &Arg) -> impl Renderable {
        let name = match a.ty {
            ArgType::NewId => "new_id",
            ArgType::Int => "int",
            ArgType::Uint => "uint",
            ArgType::Fixed => "fixed",
            ArgType::String => "string",
            ArgType::Object => "object",
            ArgType::Array => "array",
            ArgType::Fd => "fd",
        };
        let mut li = None;
        let mut le = None;
        if let Some(i) = &a.interface {
            match i {
                ArgInterface::Unknown(i) => li = Some(*i),
                ArgInterface::Known(_, i) => li = Some(*i),
            }
        } else if let Some(e) = &a.enum_ {
            match e {
                ArgEnum::Unknown(e) => le = Some(*e),
                ArgEnum::Known(_, i, e) => {
                    if interface.name != *i {
                        li = Some(*i);
                    }
                    le = Some(*e);
                }
            }
        }
        maud! {
            @if a.allow_null {
                span .arg-ty-nullable .type-modifier-hue {
                    "nullable"
                }
                " "
            }
            span .arg-ty .type-hue { (name) }
            @if li.is_some() || le.is_some() {
                "<"
                (self.wrap_in_link(a, maud! {
                    @if let Some(i) = li {
                        (i)
                    }
                    @if li.is_some() && le.is_some() {
                        "."
                    }
                    @if let Some(e) = le {
                        (e)
                    }
                }))
                ">"
            }
        }
    }

    fn enum_(&self, p: &Protocol, i: &Interface, m: &Member, e: &Enum) -> impl Renderable {
        maud! {
            (self.enum_signature(p, i, m, e))
            div .message-body {
                div .badges {
                    @if e.bitfield {
                        span .badge-normal .enum-hue { "This is a bitfield" }
                    }
                    (self.since_badge(m.since))
                }
                ul {
                    @for e in &e.entries {
                        (self.arg_description(e.name, e.summary, &e.description, e.since, e.deprecated_since))
                    }
                }
                @if let Some(v) = &m.description {
                    (self.description(v))
                }
            }
        }
    }

    fn enum_signature(&self, p: &Protocol, i: &Interface, m: &Member, e: &Enum) -> impl Renderable {
        maud! {
            pre .message-signature id=(self.member_anchor(p, i, m)) {
                span .message-interface-link {
                    a href=(self.interface_link(p, i)) { (i.name) }
                }
                "\n"
                span .keyword .enum-hue { "enum" }
                " "
                span .member-name .enum-hue {
                    a href=(self.member_link(p, i, m)) {
                        (m.name)
                    }
                }
                " "
                span .enum-paren { "{\n" }
                @for e in &e.entries {
                    "    "
                    span .arg-name { (e.name) }
                    " "
                    span .arg-equals { "=" }
                    " "
                    (e.value)
                    span .arg-comma { "," }
                    "\n"
                }
                span .enum-paren { "}" }
            }
        }
    }

    fn arg_description(
        &self,
        name: &str,
        summary: Option<&str>,
        desc: &Option<Description>,
        since: Option<u32>,
        deprecated_since: Option<u32>,
    ) -> impl Renderable {
        let any =
            summary.is_some() || desc.is_some() || since.is_some() || deprecated_since.is_some();
        maud! {
            @if any {
                li {
                    span .main-arg-name { (name) }
                    @if let Some(v) = summary {
                        " "
                        span .arg-em-dash { "—" }
                        " "
                        (v)
                    }
                    div .badges {
                        (self.since_badge(since))
                        (self.deprecated_since_badge(deprecated_since))
                    }
                    @if let Some(v) = desc {
                        (self.description(v))
                    }
                }
            }
        }
    }

    fn since_badge(&self, since: Option<u32>) -> impl Renderable {
        maud! {
            @if let Some(v) = since {
                span .badge-normal .interface-hue { "Available since version " (v) }
            }
        }
    }

    fn deprecated_since_badge(&self, since: Option<u32>) -> impl Renderable {
        maud! {
            @if let Some(v) = since {
                span .badge-normal .deprecated-hue { "Deprecated since version " (v) }
            }
        }
    }
}
