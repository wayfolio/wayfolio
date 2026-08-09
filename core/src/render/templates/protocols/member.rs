use {
    crate::{
        ast::{ArgType, MessageType},
        compendium::MemberKind,
        render::{classes::C, templates::protocols::Protocols},
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
            div .(C::message_body) {
                div .(C::badges) {
                    @if v.ty == Some(MessageType::Destructor) {
                        span .(C::badge_destructor) { "Destructor" }
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
            true => C::request_hue,
            false => C::event_hue,
        };
        let keyword = match v.is_request {
            true => "request",
            false => "event",
        };
        let paren = match v.is_request {
            true => C::request_paren,
            false => C::event_paren,
        };
        maud! {
            pre .(C::message_signature) id=(self.member_anchor(p, i, m)) {
                span .(C::message_interface_link) {
                    a href=(self.interface_link(p, i)) {
                        (i.name)
                    }
                }
                "\n"
                span .(C::keyword) .(hue) { (keyword) }
                " "
                span .(C::member_name) .(hue) {
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
                    span .(C::arg_name) { (a.name) }
                    span .(C::arg_colon) { ":" }
                    " "
                    (self.arg_type(i, a))
                    span .(C::arg_comma) { "," }
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
                span .(C::arg_link) {
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
                span .(C::arg_ty_nullable) .(C::type_modifier_hue) {
                    "nullable"
                }
                " "
            }
            span .(C::arg_ty) .(C::type_hue) { (name) }
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
            div .(C::message_body) {
                div .(C::badges) {
                    @if e.bitfield {
                        span .(C::badge_normal) .(C::enum_hue) { "This is a bitfield" }
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
            pre .(C::message_signature) id=(self.member_anchor(p, i, m)) {
                span .(C::message_interface_link) {
                    a href=(self.interface_link(p, i)) { (i.name) }
                }
                "\n"
                span .(C::keyword) .(C::enum_hue) { "enum" }
                " "
                span .(C::member_name) .(C::enum_hue) {
                    a href=(self.member_link(p, i, m)) {
                        (m.name)
                    }
                }
                " "
                span .(C::enum_paren) { "{\n" }
                @for e in &e.entries {
                    "    "
                    span .(C::arg_name) { (e.name) }
                    " "
                    span .(C::arg_equals) { "=" }
                    " "
                    (e.value)
                    span .(C::arg_comma) { "," }
                    "\n"
                }
                span .(C::enum_paren) { "}" }
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
                    span .(C::main_arg_name) { (name) }
                    @if let Some(v) = summary {
                        " "
                        span .(C::arg_em_dash) { "—" }
                        " "
                        (v)
                    }
                    div .(C::badges) {
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
                span .(C::badge_normal) .(C::interface_hue) { "Available since version " (v) }
            }
        }
    }

    fn deprecated_since_badge(&self, since: Option<u32>) -> impl Renderable {
        maud! {
            @if let Some(v) = since {
                span .(C::badge_normal) .(C::deprecated_hue) { "Deprecated since version " (v) }
            }
        }
    }
}
