use {
    crate::{
        ast::{self, ArgType},
        compendium::MemberKind,
    },
    std::fmt::Debug,
};

#[derive(Clone, Debug)]
pub struct Suite<'a> {
    pub(crate) name: &'a str,
    pub(crate) protocols: Vec<Protocol<'a>>,
}

#[derive(Clone, Debug)]
pub struct Protocol<'a> {
    pub(crate) name: &'a str,
    pub(crate) url: Option<&'a str>,
    pub(crate) copyright: Option<Copyright<'a>>,
    pub(crate) description: Option<Description<'a>>,
    pub(crate) interfaces: Vec<Interface<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Copyright<'a> {
    pub(crate) body: &'a str,
}

#[derive(Clone, Debug)]
pub(crate) struct Description<'a> {
    pub(crate) summary: Option<&'a str>,
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Interface<'a> {
    pub(crate) name: &'a str,
    pub(crate) version: u32,
    pub(crate) frozen: Option<bool>,
    pub(crate) description: Option<Description<'a>>,
    pub(crate) members: Vec<Member<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Member<'a> {
    pub(crate) name: &'a str,
    pub(crate) since: Option<u32>,
    pub(crate) deprecated_since: Option<u32>,
    pub(crate) description: Option<Description<'a>>,
    pub(crate) ty: MemberType<'a>,
}

#[derive(Clone, Debug)]
pub(crate) enum MemberType<'a> {
    Message(Message<'a>),
    Enum(Enum<'a>),
}

#[derive(Clone, Debug)]
pub(crate) struct Arg<'a> {
    pub(crate) name: &'a str,
    pub(crate) ty: ArgType,
    pub(crate) summary: Option<&'a str>,
    pub(crate) description: Option<Description<'a>>,
    pub(crate) interface: Option<ArgInterface<'a>>,
    pub(crate) allow_null: bool,
    pub(crate) enum_: Option<ArgEnum<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) enum ArgInterface<'a> {
    Unknown(&'a str),
    Known(&'a str, &'a str),
}

#[derive(Clone, Debug)]
pub(crate) enum ArgEnum<'a> {
    Unknown(&'a str),
    Known(&'a str, &'a str, &'a str),
}

#[derive(Clone, Debug)]
pub(crate) struct Entry<'a> {
    pub(crate) name: &'a str,
    pub(crate) value: &'a str,
    pub(crate) summary: Option<&'a str>,
    pub(crate) since: Option<u32>,
    pub(crate) deprecated_since: Option<u32>,
    pub(crate) description: Option<Description<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Enum<'a> {
    pub(crate) bitfield: bool,
    pub(crate) entries: Vec<Entry<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Message<'a> {
    pub(crate) is_request: bool,
    pub(crate) ty: Option<ast::MessageType>,
    pub(crate) args: Vec<Arg<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) enum Node<'a> {
    Blockquote(Blockquote<'a>),
    List(List<'a>),
    InlineCode(InlineCode<'a>),
    Delete(Delete<'a>),
    Emphasis(Emphasis<'a>),
    Link(Link<'a>),
    InternalLink(InternalLink<'a>),
    Strong(Strong<'a>),
    Text(Text),
    Code(Code),
    ListItem(ListItem<'a>),
    Paragraph(Paragraph<'a>),
}

#[derive(Clone, Debug)]
pub(crate) struct Paragraph<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Blockquote<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct List<'a> {
    pub(crate) children: Vec<Node<'a>>,
    pub(crate) ordered: bool,
    pub(crate) start: u32,
}

#[derive(Clone, Debug)]
pub(crate) struct ListItem<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Code {
    pub(crate) value: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Text {
    pub(crate) value: String,
}

#[derive(Clone, Debug)]
pub(crate) struct Emphasis<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Strong<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct InlineCode<'a> {
    pub(crate) children: Vec<Node<'a>>,
}

#[derive(Clone, Debug)]
pub(crate) struct Link<'a> {
    pub(crate) children: Vec<Node<'a>>,
    pub(crate) url: String,
    pub(crate) title: Option<String>,
}

#[derive(Clone, Debug)]
pub(crate) struct InternalLink<'a> {
    pub(crate) text: String,
    pub(crate) protocol: &'a str,
    pub(crate) interface: &'a str,
    pub(crate) member: Option<(&'a str, MemberKind)>,
}

#[derive(Clone, Debug)]
pub(crate) struct Delete<'a> {
    pub(crate) children: Vec<Node<'a>>,
}
