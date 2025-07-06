// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Object attributes module

use derive_more::{Deref, DerefMut};
use microcad_core::*;

use crate::{
    builtin::Exporter, eval::ArgumentMap, syntax::*, tuple_value, ty::QuantityType, value::*,
};

/// Export attribute, e.g. `#[export("output.svg")`
#[derive(Clone)]
pub struct ExportAttribute {
    /// Filename.
    filename: std::path::PathBuf,
    /// Exporter.
    exporter: std::rc::Rc<dyn Exporter>,
}

impl ExportAttribute {
    /// Create a new [`ExportAttribute`] with a filename and exporter.
    pub fn new(filename: std::path::PathBuf, exporter: std::rc::Rc<dyn Exporter>) -> Self {
        Self { filename, exporter }
    }
}

impl From<ExportAttribute> for Value {
    fn from(export_attribute: ExportAttribute) -> Self {
        tuple_value!(
            filename = Value::String(String::from(
                export_attribute.filename.to_str().expect("PathBuf"),
            )),
            id = Value::String(export_attribute.exporter.id().to_string())
        )
    }
}

impl std::fmt::Debug for ExportAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Export: {id} => {filename}",
            id = self.exporter.id(),
            filename = self.filename.display()
        )
    }
}

impl std::fmt::Display for ExportAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\"{filename}\" with exporter `{id}`",
            filename = self.filename.display(),
            id = self.exporter.id()
        )
    }
}

/// Render resolution when rendering things e.g. to polygons or meshes.
#[derive(Debug, Clone)]
pub enum ResolutionAttribute {
    /// Linear resolution in millimeters (Default = 0.1mm)
    Linear(Scalar),

    /// Relative resolution.
    Relative(Scalar),
}

impl Default for ResolutionAttribute {
    fn default() -> Self {
        Self::Linear(0.1)
    }
}

impl From<ResolutionAttribute> for Value {
    fn from(resolution_attribute: ResolutionAttribute) -> Self {
        match resolution_attribute {
            ResolutionAttribute::Linear(linear) => Self::Quantity(linear.into()),
            ResolutionAttribute::Relative(relative) => {
                Self::Quantity(Quantity::new(relative, QuantityType::Length))
            }
        }
    }
}

impl TryFrom<Value> for ResolutionAttribute {
    type Error = ValueError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Scalar,
            }) => Ok(ResolutionAttribute::Relative(value)),
            Value::Quantity(Quantity {
                value,
                quantity_type: QuantityType::Length,
            }) => Ok(ResolutionAttribute::Linear(value)),
            _ => Err(ValueError::CannotConvert(
                value,
                "ResolutionAttribute".to_string(),
            )),
        }
    }
}

/// An attribute for a model tree node.
#[derive(Clone, Debug)]
pub enum Attribute {
    /// Export attributes.
    Export(ExportAttribute),
    /// Color attribute.
    Color(Color),
    /// Render resolution attribute.
    Resolution(ResolutionAttribute),
    /// Exporter specific attributes.
    ExporterSpecific(Identifier, ArgumentMap),
}

impl Attribute {
    /// Return an id for the attribute.
    fn id(&self) -> Identifier {
        match &self {
            Attribute::Export(_) => Identifier::no_ref("export"),
            Attribute::Color(_) => Identifier::no_ref("color"),
            Attribute::Resolution(_) => Identifier::no_ref("resolution"),
            Attribute::ExporterSpecific(identifier, _) => identifier.clone(),
        }
    }
}

impl From<Attribute> for Value {
    fn from(value: Attribute) -> Self {
        match value {
            Attribute::Export(export_attribute) => export_attribute.into(),
            Attribute::Color(color) => Value::Tuple(Box::new(color.into())),
            Attribute::Resolution(resolution_attribute) => resolution_attribute.into(),
            Attribute::ExporterSpecific(_, argument_map) => {
                Value::Tuple(Box::new(argument_map.into()))
            }
        }
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

/// Node metadata, from an evaluated attribute list.
#[derive(Clone, Debug, Default, Deref, DerefMut)]
pub struct Attributes(Vec<Attribute>);

impl Attributes {
    /// Create new attributes from attribute.
    pub fn new(attributes: Vec<Attribute>) -> Self {
        Self(attributes)
    }
}

/// Access an attributes value by id.
pub trait GetAttribute {
    /// Get an attribute if the attribute does not exist.
    fn get_attribute(&self, id: &Identifier) -> Option<Attribute>;

    /// Get export attributes.
    fn get_export_attribute(&self) -> Option<ExportAttribute> {
        match self.get_attribute(&Identifier::no_ref("export")) {
            Some(Attribute::Export(export_attribute)) => Some(export_attribute),
            _ => None,
        }
    }

    /// Get specific exporter attributes by id.
    fn get_exporter_attribute(&self, id: &Identifier) -> Option<ArgumentMap> {
        match self.get_attribute(id) {
            Some(Attribute::ExporterSpecific(_, argument_map)) => Some(argument_map),
            _ => None,
        }
    }

    /// Value for attribute.
    ///
    /// This function is used when accessing attributes in the µcad language via
    /// the attribute access operator `#`.
    fn get_attribute_value(&self, id: &Identifier) -> Value {
        match self.get_attribute(id) {
            Some(attribute) => attribute.into(),
            None => Value::None,
        }
    }
}

impl GetAttribute for Attributes {
    fn get_attribute(&self, id: &Identifier) -> Option<Attribute> {
        self.0
            .iter()
            .find(|attribute| *id == attribute.id())
            .cloned()
    }
}
