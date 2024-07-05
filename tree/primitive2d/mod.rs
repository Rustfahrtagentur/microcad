use crate::{Node, NodeInner};

use std::convert::From;

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

impl From<Circle> for Node {
    fn from(value: Circle) -> Self {
        Node::new(NodeInner::RenderMultiPolygon(Box::new(value)))
    }
}

impl From<Rectangle> for Node {
    fn from(value: Rectangle) -> Self {
        Node::new(NodeInner::RenderMultiPolygon(Box::new(value)))
    }
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

#[cfg(test)]
mod tests {}
