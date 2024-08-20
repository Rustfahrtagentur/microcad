pub enum BooleanOp {
    Difference,
    Union,
    Xor,
    Intersection,
}

use geo::MultiPolygon;
use microcad_render::{
    geo2d::Geometry,
    tree::{Algorithm, Node, NodeInner},
    Renderable2D, Renderer2D,
};

impl Algorithm for BooleanOp {
    fn process_2d(
        &self,
        renderer: &mut dyn Renderer2D,
        parent: Node,
    ) -> Result<Node, microcad_render::Error> {
        let mut polygons = Vec::new();

        let mut new_nodes = Vec::new();

        let handle_geo2d = |g: &Geometry, polygons: &mut Vec<MultiPolygon>| match g {
            Geometry::MultiPolygon(p) => polygons.push(p.clone()),
            _ => unimplemented!("This should throw a warning"),
        };

        let handle_renderable2d =
            |renderer: &mut dyn Renderer2D,
             renderable: &dyn Renderable2D,
             polygons: &mut Vec<MultiPolygon>| match &*renderable
                .request_geometry(renderer)
                .unwrap()
            {
                Geometry::MultiPolygon(p) => polygons.push(p.clone()),
                _ => unimplemented!("This should throw a warning"),
            };
        // TODO: This is a bit of a mess, we should probably refactor this
        // first_child() must be a Group node
        for child in parent.first_child().unwrap().children() {
            let c = &*child.borrow();
            match c {
                NodeInner::Renderable2D(renderable) => {
                    handle_renderable2d(renderer, &**renderable, &mut polygons)
                }
                NodeInner::Geometry2D(g) => handle_geo2d(g, &mut polygons),
                NodeInner::Algorithm(algorithm) => {
                    new_nodes.push(algorithm.process_2d(renderer, child.clone())?)
                }
                _ => continue,
            }
        }

        for node in new_nodes {
            let c = &*node.borrow();
            match c {
                NodeInner::Geometry2D(g) => handle_geo2d(g, &mut polygons),
                NodeInner::Renderable2D(generator) => {
                    handle_renderable2d(renderer, generator.as_ref(), &mut polygons)
                }
                _ => continue,
            }
        }

        let mut result = polygons[0].clone();

        for (i, polygon) in polygons.iter().enumerate() {
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

        Ok(Node::new(NodeInner::Geometry2D(std::rc::Rc::new(
            Geometry::MultiPolygon(result),
        ))))
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
