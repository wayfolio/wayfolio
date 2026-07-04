use {
    crate::{
        ast,
        compendium::Compendium,
        tree::{
            Blockquote, Code, Delete, Description, Emphasis, InlineCode, InternalLink, Link, List,
            ListItem, Node, Paragraph, Strong, Text,
        },
    },
    isnt::std_1::primitive::IsntStrExt,
    markdown::{Constructs, ParseOptions, mdast, to_mdast},
    std::collections::HashMap,
};

pub(crate) fn analyze_description<'a>(
    compendium: &'a Compendium<'a>,
    v: &'a ast::Description,
) -> Description<'a> {
    let mdast = to_mdast(
        &normalize_body(&v.body),
        &ParseOptions {
            constructs: Constructs {
                attention: true,
                autolink: true,
                block_quote: true,
                character_escape: true,
                character_reference: true,
                code_indented: true,
                code_fenced: true,
                code_text: true,
                definition: true,
                frontmatter: false,
                gfm_autolink_literal: true,
                gfm_footnote_definition: false,
                gfm_label_start_footnote: false,
                gfm_strikethrough: true,
                gfm_table: false,
                gfm_task_list_item: false,
                hard_break_escape: true,
                hard_break_trailing: true,
                heading_atx: false,
                heading_setext: false,
                html_flow: false,
                html_text: false,
                label_start_image: false,
                label_start_link: true,
                label_end: true,
                list_item: true,
                math_flow: false,
                math_text: false,
                mdx_esm: false,
                mdx_expression_flow: false,
                mdx_expression_text: false,
                mdx_jsx_flow: false,
                mdx_jsx_text: false,
                thematic_break: false,
            },
            gfm_strikethrough_single_tilde: true,
            math_text_single_dollar: false,
            mdx_expression_parse: None,
            mdx_esm_parse: None,
        },
    )
    .unwrap();
    let mut children = vec![];
    let mut analysis = Analysis {
        compendium,
        definitions: &analyze_definitions(&mdast),
        children: &mut children,
    };
    analysis.analyze_node(mdast);
    Description {
        summary: v.summary.as_deref(),
        children,
    }
}

fn normalize_body(description: &str) -> String {
    let mut trim = None;
    let mut empty_lines = 0;
    let mut out = String::new();
    'outer: for mut line in description.lines() {
        if trim.is_none() {
            let mut spaces = 0usize;
            'spaces: {
                for c in line.chars() {
                    if c == ' ' {
                        spaces += 1;
                    } else if c == '\t' {
                        spaces = (spaces + 8) & !7;
                    } else {
                        break 'spaces;
                    }
                }
                continue 'outer;
            }
            trim = Some(spaces);
        }
        let trim = trim.unwrap();
        let mut line_buf = String::new();
        if line.contains('\t') {
            let mut offset = 0;
            for c in line.chars() {
                if c == '\t' {
                    line_buf.push(' ');
                    offset += 1;
                    let delta = (-offset) & 7;
                    for _ in 0..delta {
                        line_buf.push(' ');
                    }
                    offset += delta;
                } else {
                    line_buf.push(c);
                    offset += 1;
                }
            }
            line = &line_buf;
        }
        let idx = 'idx: {
            let mut spaces = 0usize;
            for (idx, c) in line.char_indices() {
                if spaces >= trim {
                    break 'idx idx;
                }
                if c == ' ' {
                    spaces += 1;
                } else {
                    break 'idx idx;
                }
            }
            line.len()
        };
        line = &line[idx..];
        if line.trim_ascii().is_empty() {
            empty_lines += 1;
            continue;
        }
        if empty_lines > 0 {
            for _ in 0..empty_lines {
                out.push_str("\n");
            }
            empty_lines = 0;
        }
        out.push_str(line);
        out.push_str("\n");
    }
    out
}

struct Analysis<'a, 'b> {
    compendium: &'a Compendium<'a>,
    definitions: &'b HashMap<String, mdast::Definition>,
    children: &'b mut Vec<Node<'a>>,
}

impl<'a> Analysis<'a, '_> {
    fn analyze_node(&mut self, node: mdast::Node) {
        let node = match node {
            mdast::Node::Root(v) => {
                for node in v.children {
                    self.analyze_node(node);
                }
                return;
            }
            mdast::Node::Blockquote(v) => Node::Blockquote(Blockquote {
                children: self.analyze_children(v.children),
            }),
            mdast::Node::FootnoteDefinition(_) => return,
            mdast::Node::MdxJsxFlowElement(_) => return,
            mdast::Node::List(v) => Node::List(List {
                children: self.analyze_children(v.children),
                ordered: v.ordered,
                start: v.start.unwrap_or(1),
            }),
            mdast::Node::MdxjsEsm(_) => return,
            mdast::Node::Toml(_) => return,
            mdast::Node::Yaml(_) => return,
            mdast::Node::Break(_) => return,
            mdast::Node::InlineCode(v) => {
                let children = self.analyze_nested(|analysis| analysis.analyze_text(&v.value));
                Node::InlineCode(InlineCode { children })
            }
            mdast::Node::InlineMath(_) => return,
            mdast::Node::Delete(v) => Node::Delete(Delete {
                children: self.analyze_children(v.children),
            }),
            mdast::Node::Emphasis(v) => Node::Emphasis(Emphasis {
                children: self.analyze_children(v.children),
            }),
            mdast::Node::MdxTextExpression(_) => return,
            mdast::Node::FootnoteReference(_) => return,
            mdast::Node::Html(_) => return,
            mdast::Node::Image(_) => return,
            mdast::Node::ImageReference(_) => return,
            mdast::Node::MdxJsxTextElement(_) => return,
            mdast::Node::Link(v) => {
                if is_allowed_url(&v.url) {
                    Node::Link(Link {
                        children: self.analyze_children(v.children),
                        url: v.url,
                        title: v.title,
                    })
                } else {
                    for node in v.children {
                        self.analyze_node(node);
                    }
                    return;
                }
            }
            mdast::Node::LinkReference(v) => match self.definitions.get(&*v.identifier) {
                None => {
                    for node in v.children {
                        self.analyze_node(node);
                    }
                    return;
                }
                Some(def) => Node::Link(Link {
                    children: self.analyze_children(v.children),
                    url: def.url.clone(),
                    title: def.title.clone(),
                }),
            },
            mdast::Node::Strong(v) => Node::Strong(Strong {
                children: self.analyze_children(v.children),
            }),
            mdast::Node::Text(v) => {
                self.analyze_text(&v.value);
                return;
            }
            mdast::Node::Code(v) => Node::Code(Code { value: v.value }),
            mdast::Node::Math(_) => return,
            mdast::Node::MdxFlowExpression(_) => return,
            mdast::Node::Heading(_) => return,
            mdast::Node::Table(_) => return,
            mdast::Node::ThematicBreak(_) => return,
            mdast::Node::TableRow(_) => return,
            mdast::Node::TableCell(_) => return,
            mdast::Node::ListItem(v) => Node::ListItem(ListItem {
                children: self.analyze_children(v.children),
            }),
            mdast::Node::Definition(_) => return,
            mdast::Node::Paragraph(v) => Node::Paragraph(Paragraph {
                children: self.analyze_children(v.children),
            }),
        };
        self.children.push(node);
    }

    fn analyze_text(&mut self, mut v: &str) {
        while let Some(nlr) = self.compendium.find_text_reference(v) {
            if nlr.offset > 0 {
                let prefix;
                (prefix, v) = v.split_at(nlr.offset);
                self.children.push(Node::Text(Text {
                    value: prefix.to_owned(),
                }));
            }
            let link_text;
            (link_text, v) = v.split_at(nlr.len);
            self.children.push(Node::InternalLink(InternalLink {
                text: link_text.to_owned(),
                protocol: nlr.protocol,
                interface: nlr.interface,
                member: nlr.member,
            }));
        }
        if v.is_not_empty() {
            self.children.push(Node::Text(Text {
                value: v.to_owned(),
            }));
        }
    }

    fn analyze_children(&mut self, v: Vec<mdast::Node>) -> Vec<Node<'a>> {
        self.analyze_nested(|analysis| {
            for node in v {
                analysis.analyze_node(node);
            }
        })
    }

    fn analyze_nested(&mut self, f: impl FnOnce(&mut Analysis<'a, '_>)) -> Vec<Node<'a>> {
        let mut children = vec![];
        let mut analysis = Analysis {
            compendium: self.compendium,
            definitions: self.definitions,
            children: &mut children,
        };
        f(&mut analysis);
        children
    }
}

fn analyze_definitions(v: &mdast::Node) -> HashMap<String, mdast::Definition> {
    let mut res = HashMap::new();
    analyze_definitions_(&mut res, v);
    res
}

fn analyze_definitions_(res: &mut HashMap<String, mdast::Definition>, v: &mdast::Node) {
    if let mdast::Node::Definition(v) = v {
        if is_allowed_url(&v.url) {
            res.insert(v.identifier.clone(), v.clone());
        }
    } else {
        for child in v.children().iter().copied().flatten() {
            analyze_definitions_(res, child);
        }
    }
}

fn is_allowed_url(url: &str) -> bool {
    url.starts_with("http:") || url.starts_with("https:") || url.starts_with("mailto:")
}
