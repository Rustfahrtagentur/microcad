// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) tag attributes

use derive_more::Deref;
use microcad_core::{Color, Mat3, Scalar};
use microcad_lang::{
    model::{AttributesAccess, Model},
    syntax::Identifier,
    value::{Value, ValueAccess},
};

#[derive(Debug, Clone)]
pub enum SvgTagAttribute {
    /// `marker-start` attribute, e.g. for arrow heads.
    MarkerStart(String),

    /// `marker-end` attribute, e.g. for arrow heads.
    MarkerEnd(String),

    /// Style attribute: `style = "fill: skyblue; stroke: cadetblue; stroke-width: 2;"`.
    Style {
        fill: Option<Color>,
        stroke: Option<Color>,
        stroke_width: Option<Scalar>,
    },

    /// Transform by mat3 matrix attribute.
    Transform(Mat3),

    /// Class attribute.
    Class(String),

    /// Custom attribute
    Custom(String, String),
}

impl SvgTagAttribute {
    /// Class constructor.
    pub fn class(s: &str) -> Self {
        Self::Class(s.to_string())
    }

    /// Style constructor.
    pub fn style(fill: Option<Color>, stroke: Option<Color>, stroke_width: Option<Scalar>) -> Self {
        Self::Style {
            fill,
            stroke,
            stroke_width,
        }
    }

    fn id(&self) -> &str {
        match &self {
            SvgTagAttribute::MarkerStart(_) => "marker-start",
            SvgTagAttribute::MarkerEnd(_) => "marker-end",
            SvgTagAttribute::Style {
                fill: _,
                stroke: _,
                stroke_width: _,
            } => "style",
            SvgTagAttribute::Transform(_) => "transform",
            SvgTagAttribute::Class(_) => "class",
            SvgTagAttribute::Custom(id, _) => id,
        }
    }
}

impl std::fmt::Display for SvgTagAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match &self {
            SvgTagAttribute::MarkerStart(marker_name) | SvgTagAttribute::MarkerEnd(marker_name) => {
                format!("url(#{marker_name})")
            }
            SvgTagAttribute::Style {
                fill,
                stroke,
                stroke_width,
            } => format!(
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
            SvgTagAttribute::Transform(m) => {
                let (a, b, c, d, e, f) = (m.x.x, m.x.y, m.y.x, m.y.y, m.z.x, m.z.y);
                format!("matrix({a} {b} {c} {d} {e} {f})")
            }
            SvgTagAttribute::Class(class) => class.clone(),
            SvgTagAttribute::Custom(_, value) => value.clone(),
        };

        write!(f, "{}=\"{value}\"", self.id(),)
    }
}

/// Tag attributes for an SVG tag.
#[derive(Debug, Clone, Default, Deref)]
pub struct SvgTagAttributes(std::collections::BTreeMap<String, SvgTagAttribute>);

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
    /// Insert new tag attribute.
    pub fn insert(mut self, attr: SvgTagAttribute) -> Self {
        match self.0.get_mut(attr.id()) {
            Some(SvgTagAttribute::Class(class)) => match attr {
                SvgTagAttribute::Class(new_class) => {
                    *class += &format!(" {new_class}");
                }
                _ => unreachable!(),
            },
            _ => {
                self.0.insert(attr.id().to_string(), attr);
            }
        }

        self
    }

    /// Apply SVG attributes from model attributes
    pub fn apply_from_model(mut self, model: &Model) -> Self {
        if let Some(color) = model.get_color() {
            self = self.insert(SvgTagAttribute::Style {
                fill: Some(color),
                stroke: None,
                stroke_width: None,
            });
        }

        model
            .get_custom_attributes(&Identifier::no_ref("svg"))
            .iter()
            .for_each(|tuple| {
                if let Some(Value::String(style)) = tuple.by_id(&Identifier::no_ref("style")) {
                    self = self
                        .clone()
                        .insert(SvgTagAttribute::Custom("style".into(), style.clone()));
                }
                if let Some(Value::String(fill)) = tuple.by_id(&Identifier::no_ref("fill")) {
                    self = self
                        .clone()
                        .insert(SvgTagAttribute::Custom("fill".into(), fill.clone()));
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
                .values()
                .map(|attr| attr.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )
    }
}

impl From<SvgTagAttribute> for SvgTagAttributes {
    fn from(value: SvgTagAttribute) -> Self {
        [value].into_iter().collect()
    }
}

impl FromIterator<SvgTagAttribute> for SvgTagAttributes {
    fn from_iter<T: IntoIterator<Item = SvgTagAttribute>>(iter: T) -> Self {
        let mut s = Self::default();
        iter.into_iter().for_each(|attr| {
            s.0.insert(attr.id().to_string(), attr);
        });
        s
    }
}

impl<'a> FromIterator<(&'a str, &'a str)> for SvgTagAttributes {
    fn from_iter<T: IntoIterator<Item = (&'a str, &'a str)>>(iter: T) -> Self {
        let mut s = Self::default();
        iter.into_iter().for_each(|(key, value)| {
            let key = key.to_string();
            s.0.insert(key.clone(), SvgTagAttribute::Custom(key, value.to_string()));
        });
        s
    }
}
