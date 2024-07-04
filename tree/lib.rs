mod algorithm;
mod primitive2d;

use csg_parser::syntaxtree::{SyntaxNode, SyntaxNodeKind};

pub enum NodeInner {
    Root,
    Primitive2D(Box<dyn primitive2d::Primitive2D>),
    Algorithm(Box<dyn algorithm::Algorithm>),
}

pub type Node = rctree::Node<NodeInner>;

struct TreeBuilder;

impl TreeBuilder {
    pub fn from_syntax_node(syntax_node: SyntaxNode) -> Node {
        let root = Node::new(NodeInner::Root);

        Self::_from_syntax_node(root.clone(), syntax_node);
        root
    }

    fn _from_syntax_node(parent: Node, syntax_node: SyntaxNode) {
        for child in syntax_node.children() {
            let c = child.borrow();

            match c.kind() {
                SyntaxNodeKind::ObjectNode(object_node) => {
                    let mut node = None;
                    match object_node.qualified_name().to_string().as_str() {
                        "circle" => {
                            // Todo: Parse arguments
                            let circle = primitive2d::Circle {
                                radius: 5.0,
                                points: 10,
                            };
                            node = Some(Node::new(NodeInner::Primitive2D(Box::new(circle))));
                        }
                        "rectangle" => {
                            // Todo: Create rectangle
                        }
                        _ => {}
                    }
                    if let Some(node) = node {
                        parent.append(node.clone());
                        Self::_from_syntax_node(node, child.clone());
                    }
                }
                SyntaxNodeKind::Document(_) => {} // Ignore
                _ => {}
            }
        }
    }
}
