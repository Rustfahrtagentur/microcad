// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use super::*;
use geo::coord;
use microcad_core::*;

#[test]
fn svg_writer() {
    // Write to file test.svg
    let file = std::fs::File::create("../target/svg_write.svg").expect("test error");

    let mut svg = SvgWriter::new(
        Box::new(file),
        Some(geo::Rect::new(
            geo::Point::new(0.0, 0.0),
            geo::Point::new(100.0, 100.0),
        ))
        .into(),
        1.0,
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
fn svg_sample_sketch() {
    let file = std::fs::File::create("../target/svg_sample_sketch.svg").expect("test error");

    let mut svg = SvgWriter::new(
        Box::new(file),
        Some(geo::Rect::new(
            geo::Point::new(0.0, 0.0),
            geo::Point::new(100.0, 100.0),
        ))
        .into(),
        1.0,
    )
    .expect("test error");

    let radius = 10.0;
    let width = 30.0;
    let height = 20.0;

    let rect = Rect::new(coord! {x: 0.0, y: 0.0}, coord! {x: width, y: height});
    let circle = Circle {
        radius,
        offset: Vec2::new(width, height),
    };

    /*
    svg.background(Color::from_str("white"));
    svg.grid(Grid::default());

    // Draw rectangle `r`.
    svg.rect(&rect, attr);
    svg.centered_text("r", &rect, Some("8mm".into()), None, attr);

    // Draw rectangle measures
    let offset = 15.0;
    // Height measure.
    svg.edge_measure(
        &format!("height = {height}mm"),
        Edge2D(rect.min(), rect.min() + geo::Point(0.0, rect.max().y)),
        offset,
        attr,
    );

    // Width measure.
    svg.edge_measure(
        &format!("height = {width}mm"),
        Edge2D(rect.min(), rect.min() + geo::Point(rect.max().x, 0.0)),
        offset,
        attr,
    );

    // Draw circle `c`.
    svg.circle(&circle, attr);
    svg.centered_text(
        "c",
        &circle.fetch_bounds_2d().rect().unwrap(),
        Some("8mm".into()),
        None,
        attr,
    );

    // Draw circle measures
    svg.radius_measure(&format!("radius = {radius}mm"), &circle, 45.0);

    // Draw intersection.
    let intersection = Geometry2D::Rect(rect)
        .boolean_op(
            &RenderResolution::default(),
            &Geometry2D::Circle(circle),
            &BooleanOp::Intersection,
        )
        .expect("Some geometry");

    svg.geometry(&intersection, attr);

    let bounds = geometry.fetch_bounds_2d();
    svg.size_measure("mm", bounds);

    */
}
