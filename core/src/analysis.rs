use crate::{
    analysis::description::analyze_description,
    ast,
    compendium::{Compendium, MemberKind},
    tree::{
        Arg, ArgEnum, ArgInterface, Copyright, Entry, Enum, Interface, Member, MemberType, Message,
        Protocol, Suite,
    },
};

mod description;

pub(crate) fn analyze<'a>(
    compendium: &'a Compendium<'a>,
    suites: &[&'a ast::Suite],
    protocols: &[&'a ast::Protocol],
) -> (Vec<Suite<'a>>, Vec<Protocol<'a>>) {
    let mut analysis = Analysis {
        compendium,
        interface: "",
    };
    (
        suites
            .iter()
            .copied()
            .map(|v| analysis.analyze_suite(v))
            .collect(),
        protocols
            .iter()
            .copied()
            .map(|v| analysis.analyze_protocol(v))
            .collect(),
    )
}

struct Analysis<'a> {
    compendium: &'a Compendium<'a>,
    interface: &'a str,
}

impl<'a> Analysis<'a> {
    fn analyze_suite(&mut self, v: &'a ast::Suite) -> Suite<'a> {
        Suite {
            name: &v.name,
            protocols: v
                .protocols
                .iter()
                .map(|v| self.analyze_protocol(v))
                .collect(),
        }
    }

    fn analyze_protocol(&mut self, v: &'a ast::Protocol) -> Protocol<'a> {
        Protocol {
            name: &v.name,
            url: v.url.as_deref(),
            copyright: v.copyright.as_ref().map(|v| self.analyze_copyright(v)),
            description: v
                .description
                .as_ref()
                .map(|v| analyze_description(self.compendium, v)),
            interfaces: v
                .interfaces
                .iter()
                .map(|v| self.analyze_interface(v))
                .collect(),
        }
    }

    fn analyze_copyright(&self, v: &'a ast::Copyright) -> Copyright<'a> {
        Copyright { body: &v.body }
    }

    fn analyze_interface(&mut self, v: &'a ast::Interface) -> Interface<'a> {
        self.interface = &v.name;
        Interface {
            name: &v.name,
            version: v.version,
            frozen: v.frozen,
            description: v
                .description
                .as_ref()
                .map(|v| analyze_description(self.compendium, v)),
            members: v.members.iter().map(|v| self.analyze_member(v)).collect(),
        }
    }

    fn analyze_member(&self, v: &'a ast::Member) -> Member<'a> {
        Member {
            name: &v.name,
            since: v.since,
            deprecated_since: v.deprecated_since,
            description: v
                .description
                .as_ref()
                .map(|v| analyze_description(self.compendium, v)),
            ty: match &v.ty {
                ast::MemberType::Message(v) => MemberType::Message(self.analyze_message(v)),
                ast::MemberType::Enum(v) => MemberType::Enum(self.analyze_enum(v)),
            },
        }
    }

    fn analyze_enum(&self, v: &'a ast::Enum) -> Enum<'a> {
        Enum {
            bitfield: v.bitfield,
            entries: v.entries.iter().map(|v| self.analyze_entry(v)).collect(),
        }
    }

    fn analyze_entry(&self, v: &'a ast::Entry) -> Entry<'a> {
        Entry {
            name: &v.name,
            value: &v.value,
            summary: v.summary.as_deref(),
            since: v.since,
            deprecated_since: v.deprecated_since,
            description: v
                .description
                .as_ref()
                .map(|v| analyze_description(self.compendium, v)),
        }
    }

    fn analyze_message(&self, v: &'a ast::Message) -> Message<'a> {
        Message {
            is_request: v.is_request,
            ty: v.ty,
            args: v.args.iter().map(|v| self.analyze_arg(v)).collect(),
        }
    }

    fn analyze_arg(&self, v: &'a ast::Arg) -> Arg<'a> {
        let mut interface = None;
        if let Some(name) = &v.interface {
            let i = if let Some(protocol) = self.compendium.find_protocol(name, None) {
                ArgInterface::Known(protocol, name)
            } else {
                ArgInterface::Unknown(name)
            };
            interface = Some(i);
        }
        let mut enum_ = None;
        if let Some(name) = &v.enum_ {
            let (iname, ename) = name.split_once(".").unwrap_or((self.interface, name));
            let e = if let Some(protocol) = self
                .compendium
                .find_protocol(iname, Some((ename, MemberKind::Enum)))
            {
                ArgEnum::Known(protocol, iname, ename)
            } else {
                ArgEnum::Unknown(name)
            };
            enum_ = Some(e);
        }
        Arg {
            name: &v.name,
            ty: v.ty,
            summary: v.summary.as_deref(),
            description: v
                .description
                .as_ref()
                .map(|v| analyze_description(self.compendium, v)),
            interface,
            allow_null: v.allow_null,
            enum_,
        }
    }
}
