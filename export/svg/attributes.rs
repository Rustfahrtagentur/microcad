// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) tag attributes

use derive_more::Deref;
use microcad_core::{Color, Scalar};
use microcad_lang::{
    model_tree::{GetAttribute, ModelNode},
    syntax::Identifier,
};

/// Tag attributes for an SVG tag.
#[derive(Debug, Clone, Default, Deref)]
pub struct SvgTagAttributes(std::collections::BTreeMap<String, String>);

/// Generic methods.
impl SvgTagAttributes {
    /// Merge tags with others.
    pub fn merge(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self
    }
}

/// Methods for inserting specific tag attributes.
impl SvgTagAttributes {
    fn _insert(mut self, attr: &str, value: String) -> Self {
        if !value.is_empty() {
            self.0.insert(attr.to_string(), value);
        }
        self
    }

    /// `marker-start` attribute, e.g. for arrow heads.
    pub fn marker_start(self, marker_name: &str) -> Self {
        self._insert("marker-start", format!("url(#{marker_name})"))
    }

    /// `marker-end` attribute, e.g. for arrow heads.
    pub fn marker_end(self, marker_name: &str) -> Self {
        self._insert("marker-end", format!("url(#{marker_name})"))
    }

    /// Tag for font size in millimeters.
    pub fn font_size_mm(self, font_size: Scalar) -> Self {
        self._insert("font-size", format!("{}mm", font_size as i64))
    }

    /// Fill attribute with a color.
    pub fn fill(self, color: Color) -> Self {
        self._insert("fill", color.to_svg_color())
    }

    /// Transform by mat3 matrix attribute.
    pub fn transform_matrix(self, m: &microcad_core::Mat3) -> Self {
        let (a, b, c, d, e, f) = (m.x.x, m.x.y, m.y.x, m.y.y, m.z.x, m.z.y);
        self._insert("transform", format!("matrix({a} {b} {c} {d} {e} {f})"))
    }
}

impl From<&ModelNode> for SvgTagAttributes {
    fn from(node: &ModelNode) -> Self {
        use microcad_lang::value::ValueAccess;

        match (
            node.get_exporter_attribute(&Identifier::no_ref("svg")),
            node.get_color_attribute(),
        ) {
            (None, None) => SvgTagAttributes::default(),
            (None, Some(color)) => SvgTagAttributes::default().fill(color),
            // If boths attributes are present, get style and fill from exporter attributes. Color attribute is ignored.
            (Some(attributes), None) | (Some(attributes), Some(_)) => [
                (
                    "style",
                    attributes
                        .by_id(&Identifier::no_ref("style"))
                        .map(|value| value.try_string().unwrap_or_default()),
                ),
                (
                    "fill",
                    attributes
                        .by_id(&Identifier::no_ref("fill"))
                        .map(|value| value.try_string().unwrap_or_default()),
                ),
            ]
            .into_iter()
            .collect(),
        }
    }
}

impl std::fmt::Display for SvgTagAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(attr, value)| { format!("{attr}=\"{value}\"") })
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl<'a> FromIterator<(&'a str, Option<String>)> for SvgTagAttributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, Option<String>)>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .filter_map(|(attr, value)| value.map(|value| (attr.to_string(), value)))
                .collect(),
        )
    }
}
