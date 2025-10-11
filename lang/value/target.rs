// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Type for a lookup result.

use crate::syntax::*;

/// Type for a lookup result.
#[derive(Clone, Default, PartialEq)]
pub struct Target {
    /// Name that has been looked up.
    pub name: QualifiedName,
    /// Found target name if any.
    pub target: Option<QualifiedName>,
}

impl Target {
    /// Create new target.
    pub fn new(name: QualifiedName, target: Option<QualifiedName>) -> Self {
        Self { name, target }
    }
    /// Return `true` if target is valid.
    pub fn is_valid(&self) -> bool {
        self.target.is_some()
    }

    /// Return `true` if target is not valid.
    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        if let Some(target) = &self.target {
            write!(f, "{{{name} -> {target}}}")
        } else {
            write!(f, "{{{name} -> ???}}")
        }
    }
}

impl std::fmt::Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        if let Some(target) = &self.target {
            write!(f, "{{{name:?} -> {target:?}}}")
        } else {
            color_print::cwrite!(f, "{{{name:?} -> <R!>???</>}}")
        }
    }
}
