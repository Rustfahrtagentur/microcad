// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Work piece element

use crate::{
    eval::{EvalError, EvalResult},
    model::{OutputType, Properties, PropertiesAccess},
    syntax::{Identifier, WorkbenchKind},
    value::Value,
};

/// A workpiece is an element produced by a workbench.
#[derive(Debug, Clone)]
pub struct Workpiece {
    /// Workpiece kind: `op`, `sketch`, `part`.
    pub kind: WorkbenchKind,
    /// Workpiece properties.
    pub properties: Properties,
}

impl Workpiece {
    /// Check the output type of the workpiece.
    pub fn check_output_type(&self, output_type: OutputType) -> EvalResult<()> {
        match (self.kind, output_type) {
            (WorkbenchKind::Part, OutputType::NotDetermined) => Err(EvalError::WorkbenchNoOutput(
                self.kind,
                OutputType::Geometry3D,
            )),
            (WorkbenchKind::Sketch, OutputType::NotDetermined) => Err(
                EvalError::WorkbenchNoOutput(self.kind, OutputType::Geometry2D),
            ),
            (WorkbenchKind::Part, OutputType::Geometry3D)
            | (WorkbenchKind::Sketch, OutputType::Geometry2D)
            | (WorkbenchKind::Operation, _) => Ok(()),
            (WorkbenchKind::Sketch, _) => Err(EvalError::WorkbenchInvalidOutput(
                self.kind,
                output_type,
                OutputType::Geometry2D,
            )),
            (WorkbenchKind::Part, _) => Err(EvalError::WorkbenchInvalidOutput(
                self.kind,
                output_type,
                OutputType::Geometry3D,
            )),
        }
    }
}

impl std::fmt::Display for Workpiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            WorkbenchKind::Part => write!(f, "Part"),
            WorkbenchKind::Sketch => write!(f, "Sketch"),
            WorkbenchKind::Operation => write!(f, "Operation"),
        }
    }
}

impl From<WorkbenchKind> for Workpiece {
    fn from(kind: WorkbenchKind) -> Self {
        Workpiece {
            kind,
            properties: Default::default(),
        }
    }
}

impl PropertiesAccess for Workpiece {
    fn get_property(&self, id: &Identifier) -> Option<&Value> {
        self.properties.get(id)
    }

    fn add_properties(&mut self, props: Properties) {
        self.properties
            .extend(props.iter().map(|(id, prop)| (id.clone(), prop.clone())));
    }
}
