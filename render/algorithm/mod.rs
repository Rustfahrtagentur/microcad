use crate::NodeInner;
struct Node;
pub trait Algorithm {
    fn process(&self, parent: Node) -> Node;
}

pub enum BooleanOp {
    Difference,
    Union,
    Xor,
    Intersection,
}

pub enum ProcessError {
    Unsupported,
}
