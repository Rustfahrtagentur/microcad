// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Initialization definition syntax element

use crate::{src_ref::*, syntax::*};

/// Initialization definition
///
/// Example:
///
/// ```uCAD
/// part a(a: Length) {
///     init(b: Length) { a = 2.0*b; } // The init definition
/// }
/// ```
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct InitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: Body,
    /// Source reference
    pub src_ref: SrcRef,
}

impl SrcReferrer for InitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for InitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}

impl TreeDisplay for InitDefinition {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        writeln!(f, "{:depth$}InitDefinition:", "")?;
        depth.indent();
        self.parameters.tree_print(f, depth)?;
        self.body.tree_print(f, depth)
    }
}

/// Iterator over part's init statements
pub struct Inits<'a>(std::slice::Iter<'a, Statement>);

/// Interface for elements which have initializers
pub trait Initialized<'a> {
    /// return iterator of body statements
    fn statements(&'a self) -> std::slice::Iter<'a, Statement>;

    /// Return iterator over all initializers
    fn inits(&'a self) -> Inits<'a>
    where
        Self: std::marker::Sized,
    {
        Inits::new(self)
    }
}

impl<'a> Inits<'a> {
    /// Create new init for a part
    pub fn new(def: &'a impl Initialized<'a>) -> Self {
        Self(def.statements())
    }
}

impl<'a> Iterator for Inits<'a> {
    type Item = &'a InitDefinition;

    fn next(&mut self) -> Option<Self::Item> {
        for statement in self.0.by_ref() {
            match statement {
                Statement::Init(init_definition) => {
                    return Some(init_definition);
                }
                _ => continue,
            }
        }

        None
    }
}
