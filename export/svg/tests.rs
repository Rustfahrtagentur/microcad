// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use std::str::FromStr as _;

use super::*;
use cgmath::Rad;
use geo::coord;
use microcad_core::*;

#[test]
fn svg_writer() {
    // Write to file test.svg
    let file = std::fs::File::create("../target/svg_write.svg").expect("test error");

    let mut svg = SvgWriter::new_canvas(Box::new(file), Size2D::A4.transposed().into(), None)
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
fn svg_sample_sketch() -> std::io::Result<()> {
    let file = std::fs::File::create("../target/svg_sample_sketch.svg").expect("test error");

    let mut svg = SvgWriter::new_canvas(Box::new(file), Size2D::A4.transposed().into(), None)
        .expect("test error");

    let radius = 10.0;
    let width = 30.0;
    let height = 20.0;

    let rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: width, y: height});
    let circle = Circle {
        radius,
        offset: Vec2::new(width, height),
    };

    let attr = SvgTagAttributes::default();

    Background.write_svg(
        &mut svg,
        &attr
            .clone()
            .fill(Color::from_str("white").expect("A color")),
    )?;
    Grid::default().write_svg(&mut svg, &attr)?;

    rect.write_svg(&mut svg, &attr)?;
    CenteredText {
        text: "r".into(),
        rect,
        font_size: 12.0,
    }
    .write_svg(&mut svg, &attr)?;

    // Draw rectangle measures

    // Height measure for rect.
    EdgeLengthMeasure::height(&rect, 10.0, Some("height")).write_svg(&mut svg, &attr)?;
    // Width measure for rect.
    EdgeLengthMeasure::width(&rect, 10.0, Some("width")).write_svg(&mut svg, &attr)?;

    // Draw circle `c`.
    circle.write_svg(&mut svg, &attr)?;
    CenteredText {
        text: "c".into(),
        rect: circle.fetch_bounds_2d().rect().expect("Rect"),
        font_size: 12.0,
    }
    .write_svg(&mut svg, &attr)?;

    RadiusMeasure {
        name: Some("radius".into()),
        circle: circle.clone(),
        angle: Rad(45.0),
    }
    .write_svg(&mut svg, &attr)?;

    // Draw intersection.
    let intersection = Geometry2D::Rect(rect)
        .boolean_op(
            &RenderResolution::default(),
            &Geometry2D::Circle(circle.clone()),
            &BooleanOp::Intersection,
        )
        .expect("Some geometry");

    intersection.write_svg(&mut svg, &attr)?;

    SizeMeasure::bounds(&intersection).write_svg(&mut svg, &attr)
}
