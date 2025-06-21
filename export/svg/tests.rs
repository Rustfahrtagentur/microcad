// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export tests

use std::rc::Rc;

use super::*;
use geo::coord;
use microcad_core::{geo2d::Rect, *};
use microcad_lang::{
    model_tree::{Element, Metadata, ModelNode, ObjectBuilder},
    src_ref::{Refer, SrcRef},
};

#[test]
fn svg_writer() {
    // Write to file test.svg
    let file = std::fs::File::create("svg_write.svg").expect("test error");

    let mut svg = SvgWriter::new(
        Box::new(file),
        geo::Rect::new(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0)),
        1.0,
    )
    .expect("test error");

    let rect = geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0));
    svg.rect(&rect, &SvgTagAttributes::new("fill:blue;".into()))
        .expect("test error");

    let circle = geo2d::Circle {
        radius: 10.0,
        offset: Vec2::new(50.0, 50.0),
    };
    svg.circle(&circle, &SvgTagAttributes::new("fill:red;".into()))
        .expect("test error");

    let line = (geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0));
    svg.line(
        line.0,
        line.1,
        &SvgTagAttributes::new("stroke:black;".into()),
    )
    .expect("test error");
}

#[test]
fn svg_exporter() {
    /*let root = ObjectBuilder::new(SrcRef(None))
            .build_node()
            .set_metadata(Metadata::new());

        root.append(ModelNode::new_element(Refer::none(Element::Primitive2D(
            Rc::new(Geometry2D::Rect(geo2d::Rect::new(
                coord! {x: 0.0, y:0.0},
                coord! {x: 10.0, y: 10.0},
            ))),
        ))));

        root.append(ModelNode::new_element(Refer::none(Element::Primitive2D(
            Rc::new(Geometry2D::Circle(geo2d::Circle {
                radius: 5.0,
                offset: Vec2::new(0.0, 0.0),
            })),
        ))));
    */
}
