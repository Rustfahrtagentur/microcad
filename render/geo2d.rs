use microcad_core::Scalar;

use crate::Renderer;

pub type LineString = geo::LineString<Scalar>;
pub type Polygon = geo::Polygon<Scalar>;
pub type MultiPolygon = geo::MultiPolygon<Scalar>;
pub type Rect = geo::Rect<Scalar>;
pub type Point = geo::Point<Scalar>;
pub type Geometry = geo::Geometry<Scalar>;

pub trait Generator {
    fn generate(&self, renderer: &dyn Renderer, node: crate::tree::Node) -> Geometry;
}

pub fn line_string_to_multi_polygon(line_string: LineString) -> MultiPolygon {
    MultiPolygon::new(vec![Polygon::new(line_string, vec![])])
}
