// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model attributes.

mod attributes;
mod export;
mod layer;
mod measure;
mod resolution;

use std::rc::Rc;

pub use attributes::Attributes;
pub use export::ExportCommand;
pub use layer::Layer;
pub use measure::MeasureCommand;
pub use resolution::ResolutionAttribute;

use crate::{create_tuple_value, syntax::*, value::*};

use microcad_core::{theme::Theme, Color, Size2D};

/// A custom command attribute from an exporter, e.g.: `svg = (style = "fill:none")`
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct CustomCommand {
    /// Attribute id.
    pub id: Identifier,
    /// Argument tuple.
    pub arguments: Box<Tuple>,
}

/// An attribute for a model.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum Attribute {
    /// Color attribute: `color = "red"`
    Color(Color),
    /// Render resolution attribute: `resolution = 200%`.
    Resolution(ResolutionAttribute),
    /// Theme attribute: `theme = "default/dark"`.
    Theme(Rc<Theme>),
    /// Size attribute: `size = std::A4`.
    Size(Size2D),
    /// Export command: `export = "test.svg"`.
    #[serde(skip)]
    Export(ExportCommand),
    /// Measure command: `measure = width`
    Measure(MeasureCommand),
    /// Custom non-builtin attribute with tuples: svg = (fill = "color"))
    Custom(CustomCommand),
}

impl Attribute {
    /// Return an id for the attribute.
    fn id(&self) -> Identifier {
        match &self {
            Attribute::Color(_) => Identifier::no_ref("color"),
            Attribute::Resolution(_) => Identifier::no_ref("resolution"),
            Attribute::Theme(_) => Identifier::no_ref("theme"),
            Attribute::Size(_) => Identifier::no_ref("size"),
            Attribute::Export(_) => Identifier::no_ref("export"),
            Attribute::Measure(_) => Identifier::no_ref("measure"),
            Attribute::Custom(attr) => attr.id.clone(),
        }
    }

    /// If this method returns true, the attribute can only be set once.
    pub fn is_unique(&self) -> bool {
        matches!(
            self,
            Attribute::Color(_)
                | Attribute::Resolution(_)
                | Attribute::Theme(_)
                | Attribute::Size(_)
        )
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "#[{id} = {value}]",
            id = self.id(),
            value = match &self {
                // TODO: Do not use debug outputs, implement proper Display traits instead.
                Attribute::Color(color) => format!("{color}"),
                Attribute::Resolution(resolution) => format!("{resolution:?}"),
                Attribute::Theme(theme) => theme.name.clone(),
                Attribute::Size(size) => format!("{size:?}"),
                Attribute::Export(export) => format!("{export:?}"),
                Attribute::Measure(measure) => format!("{measure:?}"),
                Attribute::Custom(command) => format!("{command:?}"),
            }
        )
    }
}

/// This trait implementation is used to access values from an attribute.
impl From<Attribute> for Value {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Color(color) => Value::Tuple(Box::new(color.into())),
            Attribute::Resolution(resolution_attribute) => resolution_attribute.into(),
            Attribute::Theme(theme) => theme.into(),
            Attribute::Size(size) => size.into(),
            Attribute::Export(e) => e.into(),
            Attribute::Measure(m) => m.into(),
            Attribute::Custom(attr) => Value::Tuple(attr.arguments.clone()),
        }
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl From<Rc<Theme>> for Value {
    fn from(theme: Rc<Theme>) -> Self {
        create_tuple_value!(
            background = theme.background,
            name = theme.name.clone(),
            filename = theme.filename.clone().unwrap_or_default()
        )
    }
}

/// Access an attributes value by id.
pub trait AttributesAccess {
    /// Get a value attribute by id.
    fn get_attributes_by_id(&self, id: &Identifier) -> Vec<Attribute>;

    /// Get a single attributes.
    fn get_single_attribute(&self, id: &Identifier) -> Option<Attribute> {
        let attributes = self.get_attributes_by_id(id);
        match attributes.len() {
            1 => attributes.first().cloned(),
            _ => None,
        }
    }

    /// Get single attribute as value.
    fn get_attribute_value(&self, id: &Identifier) -> Value {
        match self.get_single_attribute(id) {
            Some(attribute) => attribute.into(),
            None => Value::None,
        }
    }

    /// Color (builtin attribute).
    fn get_color(&self) -> Option<Color> {
        match self.get_single_attribute(&Identifier::no_ref("color")) {
            Some(value) => match value {
                Attribute::Color(color) => Some(color),
                _ => unreachable!(),
            },
            None => None,
        }
    }

    /// Color theme (builtin attribute).
    fn get_theme(&self) -> Option<std::rc::Rc<Theme>> {
        self.get_single_attribute(&Identifier::no_ref("theme"))
            .map(|attr| match attr {
                Attribute::Theme(theme) => theme,
                _ => unreachable!(),
            })
    }

    /// Get size.
    fn get_size(&self) -> Option<Size2D> {
        self.get_single_attribute(&Identifier::no_ref("size"))
            .map(|attr| match attr {
                Attribute::Size(size) => size,
                _ => unreachable!(),
            })
    }

    /// Get all export commands.
    fn get_exports(&self) -> Vec<ExportCommand> {
        self.get_attributes_by_id(&Identifier::no_ref("export"))
            .into_iter()
            .fold(Vec::new(), |mut exports, command| {
                match command {
                    Attribute::Export(export_command) => exports.push(export_command.clone()),
                    _ => unreachable!(),
                }
                exports
            })
    }

    /// Get all measure commands.
    fn get_measures(&self) -> Vec<MeasureCommand> {
        self.get_attributes_by_id(&Identifier::no_ref("measure"))
            .iter()
            .fold(Vec::new(), |mut measures, attribute| {
                match attribute {
                    Attribute::Measure(measure_command) => measures.push(measure_command.clone()),
                    _ => unreachable!(),
                }
                measures
            })
    }

    /// Get custom attributes.
    fn get_custom_attributes(&self, id: &Identifier) -> Vec<Tuple> {
        self.get_attributes_by_id(id)
            .iter()
            .fold(Vec::new(), |mut attributes, attribute| {
                match attribute {
                    Attribute::Custom(attr) => attributes.push(attr.arguments.as_ref().clone()),
                    _ => unreachable!(),
                }
                attributes
            })
    }
}
