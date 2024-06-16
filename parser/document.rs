// Intermediate Representation

use core::fmt;

pub type Node = rctree::Node<NodeKind>;
use pest::iterators::Pairs;

struct BuildDiagnostics {}

enum ParseError {
    UnknownNodeType(String),
    AccessPrivateIdentifier(String),
}

struct Expression(String);

impl From<&str> for Expression {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

enum Visibility {
    Private,
    Public,
}

struct Identifier(String);

impl Identifier {
    pub fn visibility(self) -> Visibility {
        if self.0.starts_with('_') {
            Visibility::Private
        } else {
            Visibility::Public
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

struct QualifiedName(Vec<Identifier>);

enum Type {
    Scalar,
    Length,
    Angle,
    Bool,
}

struct FunctionDeclarationArgument {
    name: Identifier,
    r#type: Type,
    default_value: Option<Expression>,
}

struct FunctionArgument {
    name: Identifier,
    value: Expression,
}

struct FunctionCall {
    name: Identifier,
    arguments: Vec<FunctionArgument>,
}

struct NodeStatement {
    name: Option<Identifier>,
    call: FunctionCall,
}

struct UseStatement {
    module: QualifiedName,
    submodules: Vec<String>,
    alias: Option<String>,
}

enum NodeKind {
    Root,
    /// E.g. circle(r = 5.0mm) or translate(x = 10.0)
    NodeStatement(NodeStatement),
    // UseStatement(UseStatement),
    // FunctionDeclaration(FunctionDeclaration),
    // ModuleDeclaration(ModuleDeclaration),
    // ParameterDeclaration(ParameterDeclaration),
    // ConstantDeclaration(Constant),
}

impl Into<Node> for NodeKind {
    fn into(self) -> Node {
        Node::new(self)
    }
}

impl fmt::Debug for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Root => write!(f, "root"),
            NodeKind::NodeStatement(node_statement) => write!(f, "{}", node_statement.call.name),
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
    fn test_node() {
        let node: Node = NodeKind::Root.into();

        // translate(x = 5.0mm)
        let translate: Node = NodeKind::NodeStatement(NodeStatement {
            name: None,
            call: FunctionCall {
                name: "translate".into(),
                arguments: vec![FunctionArgument {
                    name: "x".into(),
                    value: "5.0mm".into(),
                }],
            },
        })
        .into();
        node.append(translate.clone());

        let circle: Node = NodeKind::NodeStatement(NodeStatement {
            name: None,
            call: FunctionCall {
                name: "circle".into(),
                arguments: vec![FunctionArgument {
                    name: "r".into(),
                    value: "5.0mm".into(),
                }],
            },
        })
        .into();

        translate.append(circle);

        for child in node.descendants() {
            let c = child.borrow();
            println!("{}{:?}", "    ".repeat(child.clone().depth()), c);
        }
    }
}
