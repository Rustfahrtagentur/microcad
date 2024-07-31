pub mod algorithm;
pub mod primitive2d;

use ucad_parser::syntax_tree::{SyntaxNode, SyntaxNodeKind};

pub enum NodeInner {
    /// Root Node
    Root,

    /// The node contains a MultiPolygon
    MultiPolygon(primitive2d::MultiPolygon),

    /// The node contains a trait that renders MultiPolygon, e.g. a primitive like a circle
    RenderMultiPolygon(Box<dyn primitive2d::RenderMultiPolygon>),

    /// The node contains an algorithm that manipulates the node or its children
    Algorithm(Box<dyn algorithm::Algorithm>),
}

pub type Node = rctree::Node<NodeInner>;

pub struct TreeBuilder;

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
                SyntaxNodeKind::ModuleNode(object_node) => {
                    let mut node = None;
                    match object_node.qualified_name().to_string().as_str() {
                        "circle" => {
                            // Todo: Parse arguments
                            node = Some(crate::primitive2d::circle(5.0, 32));
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
