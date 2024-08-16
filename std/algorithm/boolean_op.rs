pub enum BooleanOp {
    Difference,
    Union,
    Xor,
    Intersection,
}

use geo::MultiPolygon;
use microcad_render::{
    geo2d::{Generator, Geometry},
    tree::{Algorithm, Node, NodeInner},
    Renderer,
};

impl Algorithm for BooleanOp {
    fn process(&self, renderer: &dyn Renderer, parent: Node) -> Node {
        let mut polygons = Vec::new();

        let mut new_nodes = Vec::new();

        let handle_geo2d = |g: &Geometry, polygons: &mut Vec<MultiPolygon>| match g {
            Geometry::MultiPolygon(p) => polygons.push(p.clone()),
            _ => unimplemented!("This should throw a warning"),
        };

        let handle_generator2d =
            |generator: &dyn Generator, node: Node, polygons: &mut Vec<MultiPolygon>| {
                match generator.generate(renderer, node) {
                    Geometry::MultiPolygon(p) => polygons.push(p),
                    _ => unimplemented!("This should throw a warning"),
                }
            };
        // TODO: This is a bit of a mess, we should probably refactor this
        // first_child() must be a Group node
        for child in parent.first_child().unwrap().children() {
            let c = &*child.borrow();
            match c {
                NodeInner::Geometry2D(g) => handle_geo2d(g, &mut polygons),
                NodeInner::Generator2D(generator) => {
                    handle_generator2d(generator.as_ref(), child.clone(), &mut polygons)
                }
                NodeInner::Algorithm(algorithm) => {
                    new_nodes.push(algorithm.process(renderer, child.clone()))
                }
                _ => continue,
            }
        }

        for node in new_nodes {
            let c = &*node.borrow();
            match c {
                NodeInner::Geometry2D(g) => handle_geo2d(g, &mut polygons),
                NodeInner::Generator2D(generator) => {
                    handle_generator2d(generator.as_ref(), node.clone(), &mut polygons)
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

        Node::new(NodeInner::Geometry2D(Box::new(Geometry::MultiPolygon(
            result,
        ))))
    }
}

pub fn difference() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Difference)))
}
