use crate::{Node, NodeInner};

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

impl Algorithm for BooleanOp {
    fn process(&self, parent: Node) -> Node {
        let mut polygons = Vec::new();

        let mut new_nodes = Vec::new();

        for child in parent.children() {
            let c = &*child.borrow();
            match c {
                NodeInner::MultiPolygon(p) => polygons.push(p.clone()),
                NodeInner::RenderMultiPolygon(render) => polygons.push(render.render()),
                NodeInner::Algorithm(algorithm) => new_nodes.push(algorithm.process(child.clone())),
                _ => continue,
            }
        }

        for node in new_nodes {
            let c = &*node.borrow();
            match c {
                NodeInner::MultiPolygon(p) => polygons.push(p.clone()),
                NodeInner::RenderMultiPolygon(render) => polygons.push(render.render()),
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

        Node::new(NodeInner::MultiPolygon(result))
    }
}

pub fn difference() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(BooleanOp::Difference)))
}

#[cfg(test)]
mod tests {

    #[test]
    fn difference() {
        use crate::NodeInner;
        use crate::{algorithm, primitive2d};

        let difference = algorithm::difference();

        difference.append(primitive2d::circle(4.0, 32));
        difference.append(primitive2d::rectangle(2.0, 2.0));
        difference.append(primitive2d::circle(2.0, 64));

        let result;

        {
            let inner = difference.borrow();
            match &*inner {
                NodeInner::Algorithm(algorithm) => {
                    result = Some(algorithm.process(difference.clone()));
                }
                _ => panic!("Node must be an algorithm"),
            }

            if let Some(result) = result {
                let inner = result.borrow();
                match &*inner {
                    NodeInner::MultiPolygon(polygon) => {
                        let mut file = std::fs::File::create("difference.svg").unwrap();

                        let svg = crate::primitive2d::svg::SvgWriter::new(
                            &mut file,
                            geo::Rect::new(geo::Point::new(-5.0, -5.0), geo::Point::new(5.0, 5.0)),
                            100.0,
                        );

                        svg.unwrap()
                            .multi_polygon(polygon, "fill:black;stroke:none;")
                            .unwrap();
                    }
                    _ => panic!("Resulting node must be a MultiPolygon"),
                }
            }
        }
    }
}
