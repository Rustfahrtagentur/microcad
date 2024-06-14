// Intermediate Representation

pub type Node = rctree::Node<NodeData>;

struct QualifiedName(Vec<String>);

struct NodeStatement {
    name: Option<String>,
}

enum NodeKind {
    Root,
    Action(NodeStatement),   // E.g. A circle node or rectangle
    Operator(NodeStatement), // E.g. translate or rotate
}

pub struct NodeData {
    pub name: String,
    pub kind: NodeKind,
}

impl NodeData {
    pub fn root() -> Node {
        Node::new(Self {
            name: "root".to_string(),
            kind: NodeKind::Root,
        })
    }

    pub fn operator(name: &str) -> Node {
        Node::new(Self {
            name: name.to_string(),
            kind: NodeKind::Operator(NodeStatement {
                name: Some(name.to_string()),
            }),
        })
    }

    pub fn action(name: &str) -> Node {
        Node::new(Self {
            name: name.to_string(),
            kind: NodeKind::Action(NodeStatement {
                name: Some(name.to_string()),
            }),
        })
    }

    pub fn depth(node: Node) -> usize {
        let mut depth = 0;
        let mut node = node.clone();
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
        let node = NodeData::root();

        let translate = NodeData::operator("translate");
        node.append(translate.clone());
        translate.append(NodeData::action("circle"));

        for child in node.descendants() {
            let c = child.borrow();
            println!(
                "{}{}",
                "    ".repeat(NodeData::depth(child.clone())),
                c.name
            );
        }
    }
}
