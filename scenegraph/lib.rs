mod polygon2d;

use csg_parser::syntaxtree::{SyntaxNode, SyntaxNodeKind};

pub enum NodeInner {
    Root,
    Shape2D(Box<dyn polygon2d::Primitive>),
}

pub type Node = rctree::Node<NodeInner>;

struct TreeBuilder;

impl TreeBuilder {
    pub fn from_syntax_node(syntax_node: SyntaxNode) -> Node {
        let root = Node::new(NodeInner::Root);

        for child in syntax_node.children() {
            let c = child.borrow();

            match c.kind() {
                SyntaxNodeKind::ObjectNode(object_node) => {}
                _ => {}
            }
        }

        root
    }

    fn _from_syntax_node(parent: Node, syntax_node: SyntaxNode) {
        let node = Node::new(NodeInner::Root);

        for child in syntax_node.children() {
            let c = child.borrow();

            match c.kind() {
                SyntaxNodeKind::ObjectNode(object_node) => {}
                _ => {}
            }
        }
    }
}
