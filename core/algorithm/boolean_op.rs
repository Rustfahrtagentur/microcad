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
    /*
    fn process_3d(
        &self,
        renderer: &mut dyn crate::render::Renderer3D,
        parent: Node,
    ) -> crate::Result<Node> {
        use crate::geo3d;
        let mut manifolds = Vec::new();

        let mut new_nodes = Vec::new();

        let handle_geo3d = |g: &geo3d::Geometry, meshes: &mut Vec<geo3d::TriangleMesh>| match g {
            geo3d::Geometry::Mesh(m) => manifolds.push(m.clone()),
            geo3d::Geometry::Manifold(m) => meshes.push(m.mesh().into()),
            _ => unimplemented!("This should throw a warning"),
        };

        let handle_renderable3d =
            |renderer: &mut dyn crate::render::Renderer3D,
             renderable: &dyn crate::render::Renderable3D,
             manifolds: &mut Vec<geo3d::Manifold>| match &*renderable
                .request_geometry(renderer)
                .unwrap()
            {
                geo3d::Geometry::Mesh(m) => manifolds.push(m.manifold().into()),
                geo3d::Geometry::Manifold(m) => manifolds.push(m),
                _ => unimplemented!("This should throw a warning"),
            };

        // TODO: This is a bit of a mess, we should probably refactor this
        // first_child() must be a Group node
        for child in parent.first_child().unwrap().children() {
            let c = &*child.borrow();
            match c {
                NodeInner::Renderable3D(renderable) => {
                    handle_renderable3d(renderer, &**renderable, &mut meshes)
                }
                NodeInner::Geometry3D(g) => handle_geo3d(g, &mut meshes),
                NodeInner::Algorithm(algorithm) => {
                    new_nodes.push(algorithm.process_3d(renderer, child.clone())?)
                }
                _ => continue,
            }
        }

        for node in new_nodes {
            let c = &*node.borrow();
            match c {
                NodeInner::Geometry2D(g) => handle_geo3d(g, &mut meshes),
                NodeInner::Renderable2D(generator) => {
                    handle_renderable3d(renderer, generator.as_ref(), &mut meshes)
                }
                _ => continue,
            }
        }

        let mut result = meshes[0].clone();

        for (i, polygon) in meshes.iter().enumerate() {
            if i == 0 {
                continue;
            }
            use geo::BooleanOps;
            match self {
                BooleanOp::Difference => result = result.difference(polygon),
                BooleanOp::Union => result = result.union(polygon),
                BooleanOp::Intersection => result = result.intersection(polygon),
                BooleanOp::Xor => result = result.xor(polygon),
            }
        }

        Ok(Node::new(NodeInner::Geometry3D(std::rc::Rc::new(
            geo3d::Geometry::Mesh(geo3d::TriangleMesh::default()),
        ))))
    }*/
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
