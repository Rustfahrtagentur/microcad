// Intermediate Representation

use crate::Rule;
use core::fmt;

pub type Node = rctree::Node<NodeKind>;
use crate::diagnostics::SourceLocation;
use crate::{CsglParser, FunctionArgument, FunctionCall, Identifier};

use pest::iterators::Pairs;

enum Error {
    IoError(std::io::Error),
    SyntaxError(Box<pest::error::Error<Rule>>),
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

impl Into<Node> for Document {
    fn into(self) -> Node {
        NodeKind::Document(self).into()
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

struct TreeBuilder();

impl TreeBuilder {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Node, Error> {
        let input = std::fs::read_to_string(&path)?;

        let root: Node = Document::from_path(path).into();
        Self::from_str(root, &input)
    }

    fn from_str(root: Node, input: &str) -> Result<Node, Error> {
        use pest::Parser;

        match CsglParser::parse(Rule::r#document, &input) {
            Ok(pairs) => {
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::object_node_statement => {
                            let pairs = pair.into_inner();
                            if let Some(node) = Self::object_node(root.clone(), pairs) {
                                root.append(node);
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                Ok(root)
            }
            Err(e) => Err(Error::SyntaxError(Box::new(e))),
        }
    }

    fn object_node(parent: Node, pairs: Pairs<Rule>) -> Option<Node> {
        let object_node_statement = CsglParser::object_node_statement(pairs.clone()).unwrap();
        let mut node = parent.clone();
        let id = object_node_statement.ident.as_ref();

        let mut first_node = None;

        for call in object_node_statement.calls {
            let object_node: Node = ObjectNode {
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

        if object_node_statement.has_inner {
            for pair in pairs {
                if pair.as_rule() == Rule::object_node_inner {
                    for inner_pair in pair.into_inner() {
                        // Fetch all object nodes
                        if inner_pair.as_rule() == Rule::object_node_statement {
                            node.clone().append(
                                Self::object_node(node.clone(), inner_pair.into_inner()).unwrap(),
                            );
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

struct ObjectNode {
    id: Option<Identifier>,
    call: FunctionCall,
}

impl From<ObjectNode> for Node {
    fn from(value: ObjectNode) -> Self {
        NodeKind::ObjectNode(value).into()
    }
}

enum NodeKind {
    Document(Document),
    /// E.g. circle(r = 5.0mm) or translate(x = 10.0)
    ObjectNode(ObjectNode),
    // UseStatement(UseStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // ModuleDeclaration(ModuleDeclaration),
    // ParameterDeclaration(ParameterDeclaration),
    // ConstantDeclaration(Constant),
}

impl NodeKind {
    fn id(&self) -> Option<&Identifier> {
        match self {
            NodeKind::ObjectNode(object_node) => object_node.id.as_ref(),
            _ => None,
        }
    }

    fn find_node_with_id(parent: Node, id: &Identifier) -> Option<Node> {
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

impl Into<Node> for NodeKind {
    fn into(self) -> Node {
        Node::new(self)
    }
}

impl fmt::Debug for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Document(doc) => write!(f, "{doc}"),
            NodeKind::ObjectNode(object_node) => {
                write!(f, "{}", object_node.call.ident)
            }
        }
    }
}

trait Depth {
    fn depth(self) -> usize;
}

impl Depth for Node {
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
        let node: Node = Document::new().into();

        // translate(x = 5.0mm)
        let translate: Node = ObjectNode {
            id: None,
            call: FunctionCall {
                ident: "translate".into(),
                function_argument_list: vec![FunctionArgument {
                    ident: "x".into(),
                    expression: "5.0mm".into(),
                }],
            },
        }
        .into();
        node.append(translate.clone());

        let circle: Node = ObjectNode {
            id: None,
            call: FunctionCall {
                ident: "circle".into(),
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
            println!("{}{:?}", "    ".repeat(child.clone().depth()), c);
        }
    }

    #[test]
    fn from_file() {
        let node: Node = Document::from_path("tests/nested.csg").into();
        assert!(node.has_children());
    }
}
