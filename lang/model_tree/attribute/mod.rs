// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Node attributes module.

mod attributes;
mod export;
mod layer;
mod measure;
mod resolution;

pub use attributes::Attributes;
pub use export::ExportAttribute;
pub use layer::Layer;
pub use measure::MeasureAttribute;
pub use resolution::ResolutionAttribute;

use crate::{syntax::*, value::*};

use microcad_core::Color;

/// An attribute for a model tree node.
#[derive(Clone, Debug)]
pub enum Attribute {
    /// Export attributes.
    Export(ExportAttribute),
    /// Color attribute.
    Color(Color),
    /// Measure attribute.
    Measure(MeasureAttribute),
    /// Render resolution attribute.
    Resolution(ResolutionAttribute),
    /// Exporter specific attributes.
    ExporterSpecific(Identifier, Tuple),
}

impl Attribute {
    /// Return an id for the attribute.
    fn id(&self) -> Identifier {
        match &self {
            Attribute::Export(_) => Identifier::no_ref("export"),
            Attribute::Color(_) => Identifier::no_ref("color"),
            Attribute::Resolution(_) => Identifier::no_ref("resolution"),
            Attribute::Measure(_) => Identifier::no_ref("measure"),
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
            Attribute::Measure(measure) => measure.into(),
            Attribute::ExporterSpecific(_, arguments) => Value::Tuple(Box::new(arguments)),
        }
    }
}

impl PartialEq for Attribute {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
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
    fn get_exporter_attribute(&self, id: &Identifier) -> Option<Tuple> {
        match self.get_attribute(id) {
            Some(Attribute::ExporterSpecific(_, arguments)) => Some(arguments),
            _ => None,
        }
    }

    /// Get color attribute.
    fn get_color_attribute(&self) -> Option<Color> {
        match self.get_attribute(&Identifier::no_ref("color")) {
            Some(Attribute::Color(color)) => Some(color),
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
