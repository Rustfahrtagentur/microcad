use crate::Rule;
use crate::{CsglParser, FunctionArgument, FunctionCall, Identifier, QualifiedName};
use core::fmt;
use pest::iterators::Pairs;
use std::fmt::write;

pub struct SyntaxNodeInner {
    inner: SyntaxNodeKind,
    //    span: pest::Span<'i>,
}

impl SyntaxNodeInner {
    pub fn id(&self) -> Option<&Identifier> {
        self.inner.id()
    }

    pub fn kind(&self) -> &SyntaxNodeKind {
        &self.inner
    }
}

pub enum SyntaxNodeKind {
    Document(Document),
    /// E.g. circle(r = 5.0mm) or translate(x = 10.0)
    ObjectNode(ObjectNode),
    UseStatement(UseStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // ModuleDeclaration(ModuleDeclaration),
    // ParameterDeclaration(ParameterDeclaration),
    // ConstantDeclaration(Constant),
}

impl SyntaxNodeKind {
    fn id(&self) -> Option<&Identifier> {
        match self {
            SyntaxNodeKind::ObjectNode(object_node) => object_node.id.as_ref(),
            _ => None,
        }
    }

    fn find_node_with_id(parent: SyntaxNode, id: &Identifier) -> Option<SyntaxNode> {
        for child in parent.children() {
            if let Some(child_id) = child.borrow().id() {
                if child_id == id {
                    return Some(child.clone());
                }
            }
        }

        None
    }
}

impl From<SyntaxNodeKind> for SyntaxNode {
    fn from(value: SyntaxNodeKind) -> Self {
        SyntaxNode::new(SyntaxNodeInner { inner: value })
    }
}

impl fmt::Debug for SyntaxNodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SyntaxNodeKind::Document(doc) => write!(f, "{doc}"),
            SyntaxNodeKind::ObjectNode(object_node) => {
                write!(f, "{}", object_node.call.qualified_name.to_string())
            }
            SyntaxNodeKind::UseStatement(use_statement) => write!(f, "{:?}", use_statement),
        }
    }
}

pub type SyntaxNode = rctree::Node<SyntaxNodeInner>;

trait AppendSyntaxNode {
    fn append_node(&mut self, node: SyntaxNode);
}

#[derive(Debug)]
enum Error {
    IoError(std::io::Error),
    SyntaxError(Box<pest::error::Error<Rule>>),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

pub struct Document {
    path: Option<std::path::PathBuf>,
}

impl Document {
    fn new() -> Self {
        Self { path: None }
    }

    fn from_path(path: impl AsRef<std::path::Path>) -> Self {
        Self {
            path: Some(std::path::PathBuf::from(path.as_ref())),
        }
    }
}

impl Into<SyntaxNode> for Document {
    fn into(self) -> SyntaxNode {
        SyntaxNodeKind::Document(self).into()
    }
}

struct TreeBuilder();

impl TreeBuilder {
    pub fn from_path(path: impl AsRef<std::path::Path>) -> Result<SyntaxNode, Error> {
        let input = std::fs::read_to_string(&path)?;

        let root: SyntaxNode = Document::from_path(path).into();
        Self::from_str(root, &input)
    }

    fn from_str(root: SyntaxNode, input: &str) -> Result<SyntaxNode, Error> {
        use pest::Parser;

        match CsglParser::parse(Rule::r#document, input) {
            Ok(mut pairs) => {
                let pairs = pairs.next().unwrap().into_inner();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::statement => {
                            let inner_pairs = pair.into_inner();
                            for inner_pair in inner_pairs {
                                match inner_pair.as_rule() {
                                    Rule::object_node_statement => {
                                        Self::object_node(root.clone(), inner_pair.into_inner());
                                        continue;
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                        Rule::EOI => continue,
                        _ => {
                            println!("{:?}", pair.as_rule());
                            unreachable!();
                        }
                    }
                }
                Ok(root)
            }
            Err(e) => Err(Error::SyntaxError(Box::new(e))),
        }
    }

    fn object_node(parent: SyntaxNode, pairs: Pairs<Rule>) -> Option<SyntaxNode> {
        let object_node_statement = CsglParser::object_node_statement(pairs.clone()).unwrap();
        let mut node = parent.clone();
        let id = object_node_statement.ident.as_ref();

        let mut first_node = None;

        for call in object_node_statement.calls {
            let object_node: SyntaxNode = ObjectNode {
                id: if id.is_some() && first_node.is_none() {
                    Some(id.unwrap().clone())
                } else {
                    None
                },
                call,
            }
            .into();
            node.append(object_node.clone());
            node = object_node; // Nest the nodes

            // Save the first node because we need to return it
            if first_node.is_none() {
                first_node = Some(node.clone());
            }
        }

        let last_node = node.clone();

        if object_node_statement.has_inner {
            for pair in pairs {
                if pair.as_rule() == Rule::object_node_inner {
                    for inner_pair in pair.into_inner() {
                        // Fetch all object nodes
                        if inner_pair.as_rule() == Rule::object_node_statement {
                            Self::object_node(last_node.clone(), inner_pair.into_inner());
                        }
                    }
                }
            }
        }

        first_node
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

pub struct ObjectNode {
    id: Option<Identifier>,
    call: FunctionCall,
}

impl ObjectNode {
    pub fn qualified_name(&self) -> &QualifiedName {
        &self.call.qualified_name
    }
}

impl From<ObjectNode> for SyntaxNode {
    fn from(value: ObjectNode) -> Self {
        SyntaxNodeKind::ObjectNode(value).into()
    }
}

pub struct UseAlias(pub QualifiedName, pub Identifier);

impl fmt::Debug for UseAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "use {:?} as {:?}", self.0, self.1)
    }
}

pub enum UseStatement {
    Use(Vec<QualifiedName>),
    UseFrom(Vec<QualifiedName>, QualifiedName),
    UseAll(Vec<QualifiedName>),
    UseAlias(UseAlias),
    UseAliasFrom(UseAlias, QualifiedName),
}

impl fmt::Debug for UseStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UseStatement::Use(qualified_names) => write!(f, "use {:?}", qualified_names),
            UseStatement::UseFrom(qualified_names, from) => {
                write!(f, "use {:?} from {:?}", qualified_names, from)
            }
            UseStatement::UseAll(qualified_names) => write!(f, "use * from {:?}", qualified_names),
            UseStatement::UseAlias(alias) => write!(f, "{:?}", alias),
            UseStatement::UseAliasFrom(alias, from) => write!(f, "{:?} from {:?}", alias, from),
        }
    }
}

trait Depth {
    fn depth(self) -> usize;
}

impl Depth for SyntaxNode {
    fn depth(self) -> usize {
        let mut depth = 0;
        let mut node = self.clone();
        while let Some(parent) = node.parent() {
            node = parent;
            depth += 1;
        }

        depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_document() {
        let node: SyntaxNode = Document::new().into();

        // translate(x = 5.0mm)
        let translate: SyntaxNode = ObjectNode {
            id: None,
            call: FunctionCall {
                qualified_name: "translate".into(),
                function_argument_list: vec![FunctionArgument {
                    ident: "x".into(),
                    expression: "5.0mm".into(),
                }],
            },
        }
        .into();
        node.append(translate.clone());

        let circle: SyntaxNode = ObjectNode {
            id: None,
            call: FunctionCall {
                qualified_name: "circle".into(),
                function_argument_list: vec![FunctionArgument {
                    ident: "r".into(),
                    expression: "5.0mm".into(),
                }],
            },
        }
        .into();

        translate.append(circle);

        for child in node.descendants() {
            let c = child.borrow();
            println!("{}{:?}", "    ".repeat(child.clone().depth()), c.inner);
        }
    }

    #[test]
    fn from_file() {
        let node = TreeBuilder::from_path("tests/nested.csg").unwrap();
        assert!(node.has_children());

        for child in node.descendants() {
            let c = child.borrow();
            println!("{}{:?}", "    ".repeat(child.clone().depth()), c.inner);
        }
    }
}
