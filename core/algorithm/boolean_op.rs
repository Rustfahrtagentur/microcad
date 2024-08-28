pub enum BooleanOp {
    Difference,
    Union,
    Xor,
    Intersection,
}

use crate::render::{Node, NodeInner, Renderer2D};
use crate::Algorithm;
use geo::OpType;

fn into_group(node: Node) -> Option<Node> {
    node.first_child().and_then(|n| {
        if let NodeInner::Group = *n.borrow() {
            Some(n.clone())
        } else {
            None
        }
    })
}

impl From<BooleanOp> for OpType {
    fn from(op: BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Xor => OpType::Xor,
        }
    }
}

impl From<&BooleanOp> for OpType {
    fn from(op: &BooleanOp) -> Self {
        match op {
            BooleanOp::Difference => OpType::Difference,
            BooleanOp::Union => OpType::Union,
            BooleanOp::Intersection => OpType::Intersection,
            BooleanOp::Xor => OpType::Xor,
        }
    }
}

impl Algorithm for BooleanOp {
    fn process_2d(&self, renderer: &mut dyn Renderer2D, parent: Node) -> crate::Result<Node> {
        let mut geos = Vec::new();

        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        for child in group.children() {
            let c = &*child.borrow();
            let geo = match c {
                NodeInner::Renderable2D(renderable) => renderable.request_geometry(renderer)?,
                NodeInner::Geometry2D(g) => g.clone(),
                NodeInner::Algorithm(algorithm) => {
                    let new_node = algorithm.process_2d(renderer, child.clone())?;
                    let c = &*new_node.borrow();
                    match c {
                        NodeInner::Geometry2D(g) => g.clone(),
                        _ => continue,
                    }
                }
                _ => continue,
            };

            geos.push(geo);
        }

        let mut result = geos[0].clone();
        for (i, geo) in geos.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(r) = result.boolean_op(geo.as_ref(), self) {
                result = std::rc::Rc::new(r)
            }
        }

        Ok(Node::new(NodeInner::Geometry2D(result)))
    }

    fn process_3d(
        &self,
        renderer: &mut dyn crate::render::Renderer3D,
        parent: Node,
    ) -> crate::Result<Node> {
        let mut geos = Vec::new();

        // all algorithm nodes are nested in a group
        let group = into_group(parent).unwrap();

        for child in group.children() {
            let c = &*child.borrow();
            let geo = match c {
                NodeInner::Renderable3D(renderable) => renderable.request_geometry(renderer)?,
                NodeInner::Geometry3D(g) => g.clone(),
                NodeInner::Algorithm(algorithm) => {
                    let new_node = algorithm.process_3d(renderer, child.clone())?;
                    let c = &*new_node.borrow();
                    match c {
                        NodeInner::Geometry3D(g) => g.clone(),
                        _ => continue,
                    }
                }
                _ => continue,
            };

            geos.push(geo);
        }

        let mut result = geos[0].clone();
        for (i, geo) in geos.iter().enumerate() {
            if i == 0 {
                continue;
            }
            if let Some(r) = result.boolean_op(geo.as_ref(), self) {
                result = std::rc::Rc::new(r)
            }
        }

        Ok(Node::new(NodeInner::Geometry3D(result)))
    }
}

pub fn difference() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Difference)))
}

pub fn union() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Union)))
}

pub fn intersection() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Intersection)))
}

pub fn xor() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Xor)))
}
