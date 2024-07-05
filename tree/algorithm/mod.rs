use std::borrow::Borrow;

use geo::{algorithm, CoordsIter};

use crate::{
    primitive2d::{self, RenderMultiPolygon},
    Node, NodeInner,
};

pub trait Algorithm {
    fn process(&self, parent: Node) -> Node;
}

struct Difference;

enum ProcessError {
    Unsupported,
}

/*impl Algorithm for Difference {
    fn process(&self, parent: Node) -> Result<Node, ProcessError> {
        let mut result = None;

        for child in parent.children() {
            let child = child.borrow();

            match *child {
                NodeInner::MultiPolygon(node) => match result {
                    Some(result) => Ok(merge(result, node)),
                    None => Ok(node),
                },
                NodeInner::Algorithm(algorithm) => Ok(algorithm.process(node)),
                _ => Err(ProcessError::Unsupported),
            }
        }

        Ok(result.unwrap_or(Node::new(NodeInner::Root)))
    }
}*/

impl Algorithm for Difference {
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
            result = result.difference(polygon);
        }

        Node::new(NodeInner::MultiPolygon(result))
    }
}

pub fn difference() -> Node {
    Node::new(NodeInner::Algorithm(Box::new(Difference)))
}

#[cfg(test)]
mod tests {
    use crate::NodeInner;

    #[test]
    fn difference() {
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
