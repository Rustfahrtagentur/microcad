use crate::identifier::{Identifier, QualifiedName};
use crate::module::UseStatement;
use crate::parser::*;

use core::fmt;

#[derive(Debug)]
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
    ModuleNode(ModuleNode),
    UseStatement(UseStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // ModuleDeclaration(ModuleDeclaration),
    // VariableDeclaration(VariableDeclaration),
    // ConstantDeclaration(Constant),
}

impl SyntaxNodeKind {
    fn id(&self) -> Option<&Identifier> {
        match self {
            SyntaxNodeKind::ModuleNode(module_node) => module_node.id.as_ref(),
            _ => None,
        }
    }

    fn _find_node_with_id(parent: SyntaxNode, id: &Identifier) -> Option<SyntaxNode> {
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
            SyntaxNodeKind::ModuleNode(module_node) => {
                write!(f, "{}", module_node.call.qualified_name)
            }
            SyntaxNodeKind::UseStatement(use_statement) => write!(f, "{}", use_statement),
        }
    }
}

pub type SyntaxNode = rctree::Node<SyntaxNodeInner>;

pub fn qualified_name(node: SyntaxNode) -> Option<QualifiedName> {
    let mut node = node.clone();
    let mut q = QualifiedName::default();

    while let Some(parent) = node.parent() {
        if let Some(id) = node.borrow().id() {
            q.push(id.clone());
        }
        node = parent;
    }

    Some(q)
}

pub trait AppendSyntaxNode {
    fn append_node(&mut self, node: SyntaxNode);
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SyntaxError(Box<pest::error::Error<Rule>>),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

#[derive(Default)]
pub struct Document {
    path: Option<std::path::PathBuf>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> Self {
        Self {
            path: Some(std::path::PathBuf::from(path.as_ref())),
        }
    }
}

impl From<Document> for SyntaxNode {
    fn from(val: Document) -> Self {
        SyntaxNodeKind::Document(val).into()
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

pub struct ModuleNode {
    id: Option<Identifier>,
    call: FunctionCall,
}

impl ModuleNode {
    pub fn qualified_name(&self) -> &QualifiedName {
        &self.call.qualified_name
    }
}

impl From<ModuleNode> for SyntaxNode {
    fn from(value: ModuleNode) -> Self {
        SyntaxNodeKind::ModuleNode(value).into()
    }
}

pub trait Depth {
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
    use crate::{expression::Expression, units::Unit};

    use super::*;

    #[test]
    fn empty_document() {
        let node: SyntaxNode = Document::new().into();

        // translate(x = 5.0mm)
        let translate: SyntaxNode = ModuleNode {
            id: None,
            call: FunctionCall {
                qualified_name: "translate".into(),
                function_argument_list: vec![FunctionArgument::NamedArgument(
                    "x".into(),
                    Expression::Literal(crate::literal::Literal::number_unit(5.0, Unit::Mm)),
                )],
            },
        }
        .into();
        node.append(translate.clone());

        // circle(r = 5.0mm)
        let circle: SyntaxNode = ModuleNode {
            id: None,
            call: FunctionCall {
                qualified_name: "circle".into(),
                function_argument_list: vec![FunctionArgument::NamedArgument(
                    "r".into(),
                    Expression::Literal(crate::literal::Literal::number_unit(5.0, Unit::Mm)),
                )],
            },
        }
        .into();

        translate.append(circle);

        for child in node.descendants() {
            let c = child.borrow();
            println!("{}{:?}", "    ".repeat(child.clone().depth()), c.inner);
        }
    }
}
