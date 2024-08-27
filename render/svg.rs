use geo::CoordsIter;
use microcad_core::{
    geo2d::*,
    render::{Node, NodeInner, Renderer, Renderer2D},
    Error, ExportSettings, Exporter, Scalar,
};

pub struct SvgWriter {
    writer: Box<dyn std::io::Write>,
}

impl SvgWriter {
    pub fn new(mut w: Box<dyn std::io::Write>, bounds: Rect, scale: f64) -> std::io::Result<Self> {
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
            writer: Box::new(w),
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

impl Drop for SvgWriter {
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

pub struct SvgRenderer {
    writer: Option<SvgWriter>,
    precision: Scalar,
    scale: Scalar,
    bounds: Rect,
    state: SvgRendererState,
}

impl SvgRenderer {
    pub fn set_output(&mut self, file: Box<dyn std::io::Write>) -> std::io::Result<()> {
        self.writer = Some(SvgWriter::new(Box::new(file), self.bounds, self.scale)?);
        Ok(())
    }

    fn writer(&mut self) -> &mut SvgWriter {
        self.writer.as_mut().unwrap()
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

impl Default for SvgRenderer {
    fn default() -> Self {
        Self {
            writer: None,
            precision: 0.1,
            bounds: Rect::new(Point::new(0.0, 0.0), Point::new(100.0, 100.0)),
            scale: 1.0,
            state: SvgRendererState::default(),
        }
    }
}

impl Renderer for SvgRenderer {
    fn precision(&self) -> Scalar {
        self.precision
    }

    fn change_render_state(&mut self, key: &str, value: &str) -> microcad_core::Result<()> {
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
}

impl Renderer2D for SvgRenderer {
    fn multi_polygon(&mut self, multi_polygon: &MultiPolygon) -> microcad_core::Result<()> {
        let style = self.render_state_to_style();
        self.writer().multi_polygon(multi_polygon, &style).unwrap();
        Ok(())
    }

    fn render_node(&mut self, node: Node) -> microcad_core::Result<()> {
        let inner = node.borrow();
        match &*inner {
            NodeInner::Export(_) | NodeInner::Group | NodeInner::Root => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
            }
            NodeInner::Algorithm(algorithm) => {
                let new_node = algorithm.process_2d(self, node.clone())?;
                self.render_node(new_node)?;
            }
            NodeInner::Renderable2D(renderable) => {
                renderable.render_geometry(self)?;
            }
            NodeInner::Geometry2D(geometry) => self.render_geometry(geometry)?,
            NodeInner::Transform(_) => unimplemented!(),
            _ => return Err(Error::NotImplemented),
        };

        Ok(())
    }
}

impl Exporter for SvgRenderer {
    fn from_settings(settings: &ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        if let Some(filename) = settings.filename() {
            let file = std::fs::File::create(filename)?;
            let mut renderer = SvgRenderer::default();
            renderer.set_output(Box::new(file))?;
            Ok(renderer)
        } else {
            Err(Error::NoFilenameSpecifiedForExport)
        }
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["svg"]
    }

    fn export(&mut self, node: Node) -> microcad_core::Result<()> {
        self.render_node(node)
    }
}

#[test]
fn svg_write() {
    // Write to file test.svg
    let file = std::fs::File::create("svg_write.svg").unwrap();

    let mut svg = SvgWriter::new(
        Box::new(file),
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
