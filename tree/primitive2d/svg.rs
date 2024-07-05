use std::io::Write;

use geo::CoordsIter;

use super::*;

pub struct SvgWriter<'a> {
    writer: &'a mut dyn Write,
    bounds: Rect,
    scale: Scalar,
}

impl<'a> SvgWriter<'a> {
    pub fn new(mut w: &'a mut dyn Write, bounds: Rect, scale: f64) -> std::io::Result<Self> {
        writeln!(&mut w, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            &mut w,
            "<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            bounds.min().x * scale,
            bounds.min().y * scale,
            bounds.width() * scale,
            bounds.height() * scale
        )?;
        writeln!(&mut w, "<g transform='scale({scale})'>")?;

        Ok(Self {
            writer: w,
            bounds,
            scale,
        })
    }

    pub fn rect(&mut self, rect: &Rect, style: &str) -> std::io::Result<()> {
        let x = rect.min().x;
        let y = rect.min().y;
        let width = rect.width();
        let height = rect.height();
        writeln!(
            self.writer,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" style=\"{style}\"/>"
        )
    }

    pub fn circle(&mut self, center: &Point, radius: f64, style: &str) -> std::io::Result<()> {
        let (cx, cy) = center.x_y();
        writeln!(
            self.writer,
            "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{radius}\" style=\"{style}\"/>"
        )
    }

    pub fn line(&mut self, p1: Point, p2: Point, style: &str) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (p1.x_y(), p2.x_y());
        writeln!(
            self.writer,
            "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" style=\"{style}\"/>"
        )
    }

    pub fn polygon(&mut self, polygon: &Polygon, style: &str) -> std::io::Result<()> {
        write!(self.writer, "<path d=\"")?;
        for (i, point) in polygon.exterior().points().enumerate() {
            let (x, y) = point.x_y();
            match i {
                0 => write!(self.writer, "M")?,
                _ => write!(self.writer, "L")?,
            }

            write!(self.writer, "{x},{y}", x = x, y = y)?;
            if i == polygon.exterior().coords_count() - 1 {
                write!(self.writer, " Z ")?;
            }
        }
        for interior in polygon.interiors() {
            for (i, point) in interior.points().enumerate() {
                let (x, y) = point.x_y();
                match i {
                    0 => write!(self.writer, "M")?,
                    _ => write!(self.writer, "L")?,
                }

                write!(self.writer, "{x},{y}", x = x, y = y)?;
                if i == interior.coords_count() - 1 {
                    write!(self.writer, " Z ")?;
                }
            }
        }

        writeln!(self.writer, "\" style=\"{style}\"/>")
    }

    pub fn multi_polygon(
        self: &mut Self,
        multi_polygon: &MultiPolygon,
        style: &str,
    ) -> std::io::Result<()> {
        for polygon in multi_polygon {
            self.polygon(polygon, style)?;
        }
        Ok(())
    }
}

impl<'a> Drop for SvgWriter<'a> {
    fn drop(&mut self) {
        writeln!(self.writer, "</g>").unwrap();
        writeln!(self.writer, "</svg>").unwrap();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn svg_write() {
        // Write to file test.svg
        let mut file = std::fs::File::create("svg_write.svg").unwrap();

        let mut svg = super::SvgWriter::new(
            &mut file,
            geo::Rect::new(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0)),
            1.0,
        )
        .unwrap();

        let rect = geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0));
        svg.rect(&rect, "fill:blue;").unwrap();

        let circle = geo::Point::new(50.0, 50.0);
        svg.circle(&circle, 10.0, "fill:red;").unwrap();

        let line = (geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0));
        svg.line(line.0, line.1, "stroke:black;").unwrap();

        use super::RenderMultiPolygon;
        let circle_polygon = super::Circle {
            radius: 40.0,
            points: 32,
        }
        .render();
        svg.multi_polygon(&circle_polygon, "fill:none;stroke:black;")
            .unwrap();
    }
}
