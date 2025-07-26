// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use std::str::FromStr as _;

use super::*;
use geo::coord;
use microcad_core::{theme::Theme, *};

#[test]
fn svg_writer() {
    // Write to file test.svg
    let file = std::fs::File::create("../target/svg_write.svg").expect("test error");

    let mut svg = SvgWriter::new_canvas(
        Box::new(file),
        Size2D::A4.transposed().into(),
        Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 100.0, y: 100.0}),
        None,
    )
    .expect("test error");

    geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0))
        .write_svg(
            &mut svg,
            &[("style", Some("fill:blue;".into()))].into_iter().collect(),
        )
        .expect("test error");

    geo2d::Circle {
        radius: 10.0,
        offset: Vec2::new(50.0, 50.0),
    }
    .write_svg(
        &mut svg,
        &[("style", Some("fill:red;".into()))].into_iter().collect(),
    )
    .expect("test error");

    Edge2D(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0))
        .write_svg(
            &mut svg,
            &[("style", Some("stroke:black;".into()))]
                .into_iter()
                .collect(),
        )
        .expect("test error");

    Edge2D(geo::Point::new(100.0, 0.0), geo::Point::new(0.0, 100.0))
        .shorter(6.0)
        .write_svg(
            &mut svg,
            &[("style", Some("stroke:black;".into()))]
                .into_iter()
                .collect(),
        )
        .expect("test error");
}

#[test]
fn svg_canvas() -> std::io::Result<()> {
    let file = std::fs::File::create("../target/svg_canvas.svg").expect("test error");

    let content_rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 100.0, y: 100.0});
    let mut svg = SvgWriter::new_canvas(
        Box::new(file),
        Size2D::A4.transposed().into(),
        content_rect,
        Some(2.0),
    )
    .expect("test error");

    assert_eq!(svg.canvas().scale(), 2.0); // The content is 100mm and the canvas height is 200mm.

    eprintln!("{:#?}", svg.canvas());

    content_rect.write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(
            None,
            Some(Color::from_str("black").expect("Black color")),
            Some(1.0),
        ),
    )?;

    [
        (0.0, 0.0),
        (0.0, 100.0),
        (100.0, 0.0),
        (100.0, 100.0),
        (50.0, 50.0),
    ]
    .iter()
    .map(|p| Circle {
        radius: 2.0,
        offset: Vec2::new(p.0, p.1),
    })
    .try_for_each(|c| {
        c.write_svg_mapped(
            &mut svg,
            &SvgTagAttributes::default().style(
                Some(Color::from_str("blue").expect("Color")),
                None,
                None,
            ),
        )?;

        let p = Point::new(c.offset.x, c.offset.y);

        CenteredText {
            text: format!("({}mm,{}mm)", p.x(), p.y()),
            rect: Rect::new(p, p),
            font_size: 1.0,
        }
        .write_svg_mapped(
            &mut svg,
            &SvgTagAttributes::default().style(
                Some(Color::from_str("gray").expect("Color")),
                None,
                None,
            ),
        )
    })?;

    Ok(())
}

#[test]
fn svg_sample_sketch() -> std::io::Result<()> {
    let file = std::fs::File::create("../target/svg_sample_sketch.svg").expect("test error");

    let content_rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: 50.0, y: 50.0});
    let mut svg = SvgWriter::new_canvas(
        Box::new(file),
        Size2D::A4.transposed().into(),
        content_rect,
        Some(3.0),
    )
    .expect("test error");

    let radius = 10.0;
    let width = 30.0;
    let height = 20.0;

    let theme = Theme::default();

    let rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: width, y: height});
    let circle = Circle {
        radius,
        offset: Vec2::new(width, height),
    };

    Background.write_svg(
        &mut svg,
        &SvgTagAttributes::default().fill(theme.background),
    )?;
    Grid::default().write_svg(
        &mut svg,
        &SvgTagAttributes::default().style(None, Some(theme.grid), Some(0.2)),
    )?;

    rect.write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(None, Some(theme.inactive), Some(0.4)),
    )?;

    CenteredText {
        text: "r".into(),
        rect,
        font_size: 4.0,
    }
    .write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(Some(theme.inactive), None, None),
    )?;

    // Draw rectangle measures

    // Height measure for rect.
    EdgeLengthMeasure::height(&rect, 10.0, Some("height")).write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(
            Some(theme.measure.make_transparent(0.5)),
            Some(theme.measure.make_transparent(0.5)),
            Some(0.2),
        ),
    )?;
    // Width measure for rect.
    EdgeLengthMeasure::width(&rect, 10.0, Some("width")).write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(
            Some(theme.measure.make_transparent(0.5)),
            Some(theme.measure.make_transparent(0.5)),
            Some(0.2),
        ),
    )?;

    // Draw circle `c`.
    circle.write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(None, Some(theme.inactive), Some(0.4)),
    )?;
    CenteredText {
        text: "c".into(),
        rect: circle.fetch_bounds_2d().rect().expect("Rect"),
        font_size: 4.0,
    }
    .write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(Some(theme.inactive), None, None),
    )?;

    RadiusMeasure::new(circle.clone(), Some("radius".into()), None).write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(
            Some(theme.measure.make_transparent(0.5)),
            Some(theme.measure.make_transparent(0.5)),
            Some(0.2),
        ),
    )?;

    // Draw intersection.
    let intersection = Geometry2D::Rect(rect)
        .boolean_op(
            &RenderResolution::default(),
            &Geometry2D::Circle(circle.clone()),
            &BooleanOp::Intersection,
        )
        .expect("Some geometry");

    intersection.write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(None, Some(theme.active), Some(0.4)),
    )?;

    SizeMeasure::bounds(&intersection).write_svg_mapped(
        &mut svg,
        &SvgTagAttributes::default().style(Some(theme.measure), Some(theme.measure), Some(0.2)),
    )?;

    Ok(())
}
