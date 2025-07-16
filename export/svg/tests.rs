// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use super::*;
use microcad_core::*;

#[test]
fn svg_writer() {
    // Write to file test.svg
    let file = std::fs::File::create("svg_write.svg").expect("test error");

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

    let rect = geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0));
    svg.rect(
        &rect,
        &SvgTagAttributes {
            style: Some("fill:blue;".into()),
            fill: None,
        },
    )
    .expect("test error");

    let circle = geo2d::Circle {
        radius: 10.0,
        offset: Vec2::new(50.0, 50.0),
    };
    svg.circle(
        &circle,
        &SvgTagAttributes {
            style: Some("fill:red;".into()),
            fill: None,
        },
    )
    .expect("test error");

    let line = Edge2D(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0));
    svg.edge(
        &line,
        &SvgTagAttributes {
            style: Some("stroke:black;".into()),
            fill: None,
        },
        None,
        None,
    )
    .expect("test error");

    let arrow = Edge2D(geo::Point::new(100.0, 0.0), geo::Point::new(0.0, 100.0));
    svg.edge(
        &arrow.shorter(6.0),
        &SvgTagAttributes {
            style: Some("stroke:black;".into()),
            fill: None,
        },
        Some("arrow".into()),
        Some("arrow".into()),
    )
    .expect("test error");
}
