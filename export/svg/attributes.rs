// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) tag attributes

use derive_more::Deref;
use microcad_core::{Color, Scalar};
use microcad_lang::{
    model::{AttributesAccess, Model},
    syntax::Identifier,
    value::{Value, ValueAccess},
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

    /// Style attribute: `style = fill: skyblue; stroke: cadetblue; stroke-width: 2;`.
    pub fn style(
        self,
        fill: Option<Color>,
        stroke: Option<Color>,
        stroke_width: Option<Scalar>,
    ) -> Self {
        self._insert(
            "style",
            format!(
                "{fill}{stroke}{stroke_width}",
                fill = match fill {
                    Some(fill) => format!("fill: {}; ", fill.to_svg_color()),
                    None => "fill: none; ".into(),
                },
                stroke = match stroke {
                    Some(stroke) => format!("stroke: {}; ", stroke.to_svg_color()),
                    None => "stroke: none; ".into(),
                },
                stroke_width = match stroke_width {
                    Some(stroke_width) => format!("stroke-width: {stroke_width}"),
                    None => String::new(),
                }
            ),
        )
    }

    /// Transform by mat3 matrix attribute.
    pub fn transform_matrix(self, m: &microcad_core::Mat3) -> Self {
        let (a, b, c, d, e, f) = (m.x.x, m.x.y, m.y.x, m.y.y, m.z.x, m.z.y);
        self._insert("transform", format!("matrix({a} {b} {c} {d} {e} {f})"))
    }

    /// Apply SVG attributes from model attributes
    pub fn apply_from_model(mut self, model: &Model) -> Self {
        if let Some(color) = model.get_color() {
            self = self.fill(color);
        }

        model
            .get_custom_attributes(&Identifier::no_ref("svg"))
            .iter()
            .for_each(|tuple| {
                if let Some(Value::String(style)) = tuple.by_id(&Identifier::no_ref("style")) {
                    self = self.clone()._insert("style", style.clone());
                }
                if let Some(Value::String(style)) = tuple.by_id(&Identifier::no_ref("fill")) {
                    self = self.clone()._insert("fill", style.clone());
                }
            });
        self
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
