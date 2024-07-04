use crate::Node;

pub trait Algorithm {
    fn render(&self, parent: Node);
}

struct Union;

impl Algorithm for Union {
    fn render(&self, parent: Node) {}
}
