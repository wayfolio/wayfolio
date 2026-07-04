//! Rust representation of Wayland protocols.
//!
//! An AST can be created with [the XML parser](parser::parse_xml).

pub mod parser;

/// A collection of protocols.
///
/// Suites are only used to group protocols together in the table of contents, i.e., the
/// index shown at the top of single.html or protocols.html.
#[derive(Clone, Debug)]
pub struct Suite {
    /// The display name of the suite.
    pub name: String,
    /// The protocols that make up the suite.
    pub protocols: Vec<Protocol>,
}

/// A protocol, corresponding to a `<protocol>` element.
#[derive(Clone, Debug)]
pub struct Protocol {
    /// The protocol name, e.g. `wayland` or `xdg_shell`.
    pub name: String,
    /// An optional URL to the protocol's upstream source.
    ///
    /// This is not part of the XML format and is not set by the XML parser.
    pub url: Option<String>,
    /// The copyright notice.
    pub copyright: Option<Copyright>,
    /// The protocol-level description.
    pub description: Option<Description>,
    /// The interfaces defined by the protocol.
    pub interfaces: Vec<Interface>,
}

/// A copyright notice, corresponding to a `<copyright>` element.
#[derive(Clone, Debug)]
pub struct Copyright {
    /// The verbatim text of the notice.
    pub body: String,
}

/// A description, corresponding to a `<description>` element.
#[derive(Clone, Debug)]
pub struct Description {
    /// The summary, taken from the element's `summary` attribute.
    pub summary: Option<String>,
    /// The body text, interpreted as Markdown during analysis.
    pub body: String,
}

/// An interface, corresponding to an `<interface>` element.
#[derive(Clone, Debug)]
pub struct Interface {
    /// The interface name, e.g. `wl_surface`.
    pub name: String,
    /// The current version of the interface.
    pub version: u32,
    /// Whether the interface is frozen, from the `frozen` attribute.
    pub frozen: Option<bool>,
    /// The interface-level description.
    pub description: Option<Description>,
    /// The requests, events, and enums defined by the interface.
    pub members: Vec<Member>,
}

/// A member of an interface: a request, an event, or an enum.
#[derive(Clone, Debug)]
pub struct Member {
    /// The member name.
    pub name: String,
    /// The interface version this member was introduced in.
    pub since: Option<u32>,
    /// The interface version this member was deprecated in.
    pub deprecated_since: Option<u32>,
    /// The member's description.
    pub description: Option<Description>,
    /// What kind of member this is, and its contents.
    pub ty: MemberType,
}

/// The kind and contents of a [`Member`].
#[derive(Clone, Debug)]
pub enum MemberType {
    /// A request or an event; the two are distinguished by
    /// [`Message::is_request`].
    Message(Message),
    /// An enum.
    Enum(Enum),
}

/// An argument of a message, corresponding to an `<arg>` element.
#[derive(Clone, Debug)]
pub struct Arg {
    /// The argument name.
    pub name: String,
    /// The type of the argument.
    pub ty: ArgType,
    /// The `summary` attribute.
    pub summary: Option<String>,
    /// The description.
    pub description: Option<Description>,
    /// For `object` and `new_id` arguments, the name of the referenced interface.
    pub interface: Option<String>,
    /// The `allow-null` attribute.
    pub allow_null: bool,
    /// For integer arguments, the optional name of the enum.
    pub enum_: Option<String>,
}

/// The type of an [`Arg`], from the `type` attribute of an `<arg>`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ArgType {
    /// A `new_id`.
    NewId,
    /// An `int`.
    Int,
    /// A `uint`.
    Uint,
    /// A `fixed`.
    Fixed,
    /// A `string`.
    String,
    /// An `object`.
    Object,
    /// An `array`.
    Array,
    /// An `fd`.
    Fd,
}

/// The `type` attribute of a request/event.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum MessageType {
    /// A `destructor`.
    Destructor,
}

/// An entry of an enum, corresponding to an `<entry>` element.
#[derive(Clone, Debug)]
pub struct Entry {
    /// The entry name.
    pub name: String,
    /// The entry's value.
    pub value: String,
    /// The summary.
    pub summary: Option<String>,
    /// The interface version this entry was introduced in.
    pub since: Option<u32>,
    /// The interface version this entry was deprecated in.
    pub deprecated_since: Option<u32>,
    /// The description.
    pub description: Option<Description>,
}

/// An enum definition, corresponding to an `<enum>` element.
#[derive(Clone, Debug)]
pub struct Enum {
    /// Whether the enum is a bitfield, from the `bitfield` attribute.
    pub bitfield: bool,
    /// The entries.
    pub entries: Vec<Entry>,
}

/// A request or an event, corresponding to a `<request>` or `<event>` element.
#[derive(Clone, Debug)]
pub struct Message {
    /// `true` for a request, `false` for an event.
    pub is_request: bool,
    /// The type.
    pub ty: Option<MessageType>,
    /// The arguments.
    pub args: Vec<Arg>,
}
