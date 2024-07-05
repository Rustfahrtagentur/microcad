use crate::{Node, NodeInner};

pub type Scalar = f64;
pub type LineString = geo::LineString<Scalar>;
pub type Polygon = geo::Polygon<Scalar>;
pub type MultiPolygon = geo::MultiPolygon<Scalar>;
pub type Rect = geo::Rect<Scalar>;
pub type Point = geo::Point<Scalar>;

pub mod svg;

pub trait RenderMultiPolygon {
    fn render(&self) -> MultiPolygon;
}

pub struct Circle {
    pub radius: f64,
    pub points: usize,
}

fn line_string_to_multi_polygon(line_string: LineString) -> MultiPolygon {
    MultiPolygon::new(vec![Polygon::new(line_string, vec![])])
}

impl RenderMultiPolygon for Circle {
    fn render(&self) -> MultiPolygon {
        let mut points = Vec::new();
        for i in 0..self.points {
            let angle = 2.0 * std::f64::consts::PI * (i as f64) / (self.points as f64);
            points.push(geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin()));
        }

        line_string_to_multi_polygon(LineString::new(points))
    }
}

pub fn circle(radius: f64, points: usize) -> Node {
    Node::new(NodeInner::RenderMultiPolygon(Box::new(Circle {
        radius,
        points,
    })))
}

pub fn rectangle(width: f64, height: f64) -> Node {
    Node::new(NodeInner::RenderMultiPolygon(Box::new(Rectangle {
        width,
        height,
    })))
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl RenderMultiPolygon for Rectangle {
    fn render(&self) -> MultiPolygon {
        use geo::line_string;
        let line_string = geo::line_string![
            (x: 0.0, y: 0.0),
            (x: self.width, y: 0.0),
            (x: self.width, y: self.height),
            (x: 0.0, y: self.height),
            (x: 0.0, y: 0.0),
        ];

        line_string_to_multi_polygon(line_string)
    }
}

/*struct Difference {
    primitives: Vec<Box<dyn RenderMultiPolygon>>,
}

impl RenderMultiPolygon for Difference {
    fn render(&self) -> MultiPolygon {
        let mut polygons = Vec::new();
        for primitive in &self.primitives {
            polygons.push(primitive.render());
        }

        let mut result = polygons[0].clone();
        for polygon in polygons.iter().skip(1) {
            use geo::BooleanOps;
            result = result.difference(polygon);
        }

        result
    }
}
*/

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn difference() {
        let circle1 = Circle {
            radius: 1.0,
            points: 32,
        };
        let circle2 = Circle {
            radius: 0.5,
            points: 32,
        };
        /*
                let difference = Difference {
                    primitives: vec![Box::new(circle1), Box::new(circle2)],
                };

                let result = difference.render();
                let mut file = std::fs::File::create("difference.svg").unwrap();

                let svg = svg::SvgWriter::new(
                    &mut file,
                    geo::Rect::new(geo::Point::new(-2.0, -2.0), geo::Point::new(2.0, 2.0)),
                    100.0,
                );

                svg.unwrap()
                    .multi_polygon(&result, "fill:none;stroke:black;")
                    .unwrap();
        println!("{:?}", result);
        */
    }
}
