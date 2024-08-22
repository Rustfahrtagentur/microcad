use super::*;
use geo::CoordsIter;
use geo2d::*;
use std::io::Write;
use tree::NodeInner;

pub struct SvgWriter<'a> {
    writer: &'a mut dyn Write,
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

        Ok(Self { writer: w })
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
        &mut self,
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

#[derive(Default)]
pub struct SvgRendererState {
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<Scalar>,
}

pub struct SvgRenderer<'a> {
    writer: SvgWriter<'a>,
    precision: Scalar,

    state: SvgRendererState,
}

impl<'a> SvgRenderer<'a> {
    pub fn new(w: &'a mut dyn Write) -> std::io::Result<Self> {
        Ok(Self {
            writer: SvgWriter::new(
                w,
                geo::Rect::new(geo::Point::new(-10.0, -10.0), geo::Point::new(10.0, 10.0)),
                1.0,
            )?,
            precision: 0.1,
            state: SvgRendererState::default(),
        })
    }

    fn render_state_to_style(&self) -> String {
        let mut style = String::new();
        if let Some(fill) = &self.state.fill {
            style.push_str(&format!("fill:{};", fill));
        }
        if let Some(stroke) = &self.state.stroke {
            style.push_str(&format!("stroke:{};", stroke));
        }
        if let Some(stroke_width) = self.state.stroke_width {
            style.push_str(&format!("stroke-width:{};", stroke_width));
        }
        style
    }
}

impl<'a> Renderer2D for SvgRenderer<'a> {
    fn precision(&self) -> Scalar {
        self.precision
    }

    fn change_render_state(&mut self, key: &str, value: &str) -> Result<(), Error> {
        match key {
            "fill" => self.state.fill = Some(value.to_string()),
            "stroke" => self.state.stroke = Some(value.to_string()),
            "stroke-width" => {
                self.state.stroke_width = Some(value.parse().unwrap());
            }
            _ => return Err(Error::NotImplemented),
        }
        Ok(())
    }

    fn multi_polygon(&mut self, multi_polygon: &geo2d::MultiPolygon) -> Result<(), Error> {
        self.writer
            .multi_polygon(multi_polygon, &self.render_state_to_style())
            .unwrap();
        Ok(())
    }

    fn render_node(&mut self, node: Node) -> Result<(), Error> {
        let inner = node.borrow();
        match &*inner {
            NodeInner::Export(_) | NodeInner::Group | NodeInner::Root => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
                return Ok(());
            }
            NodeInner::Algorithm(algorithm) => {
                let new_node = algorithm.process_2d(self, node.clone())?;
                self.render_node(new_node)?;
            }
            NodeInner::Renderable2D(renderable) => {
                renderable.render_geometry(self)?;
                return Ok(());
            }
            NodeInner::Geometry2D(geometry) => self.render_geometry(geometry)?,
            NodeInner::Transform(_) => unimplemented!(),
            NodeInner::RenderStateChange(_) => unimplemented!(),
        };

        Ok(())
    }
}

#[test]
fn svg_write() {
    // Write to file test.svg
    let mut file = std::fs::File::create("svg_write.svg").unwrap();

    let mut svg = SvgWriter::new(
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
}
