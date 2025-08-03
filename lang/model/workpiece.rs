// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Work piece element

use crate::{
    model::{Properties, PropertiesAccess},
    syntax::{Identifier, WorkbenchKind},
    value::Value,
};

/// A workpiece is an element produced by a workbench.
#[derive(Debug, Clone)]
pub struct Workpiece {
    /// Workpiece kind: `op`, `sketch`, `part`.
    kind: WorkbenchKind,
    /// Workpiece properties.
    properties: Properties,
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

impl From<WorkbenchKind> for Workpiece {
    fn from(kind: WorkbenchKind) -> Self {
        Workpiece {
            kind,
            properties: Default::default(),
        }
    }
}
