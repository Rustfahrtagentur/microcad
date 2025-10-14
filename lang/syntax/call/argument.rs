// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! A single argument

use crate::{ord_map::*, src_ref::*, syntax::*};

/// Argument in a [`Call`].
#[derive(Clone, PartialEq)]
pub struct Argument {
    /// Name of the argument
    pub id: Option<Identifier>,
    /// Value of the argument
    pub expression: Expression,
    /// Source code reference
    pub src_ref: SrcRef,
}

impl Argument {
    /// Returns the name, if self.name is some. If self.name is None, try to extract the name from the expression
    pub fn derived_name(&self) -> Option<Identifier> {
        match &self.id {
            Some(name) => Some(name.clone()),
            None => self.expression.single_identifier().cloned(),
        }
    }
}

impl SrcReferrer for Argument {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl OrdMapValue<Identifier> for Argument {
    fn key(&self) -> Option<Identifier> {
        self.id.clone()
    }
}

impl std::fmt::Display for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id} = {}", self.expression),
            None => write!(f, "{}", self.expression),
        }
    }
}

impl std::fmt::Debug for Argument {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.id {
            Some(ref id) => write!(f, "{id:?} = {:?}", self.expression),
            None => write!(f, "{:?}", self.expression),
        }
    }
}

impl TreeDisplay for Argument {
    fn tree_print(&self, f: &mut std::fmt::Formatter, mut depth: TreeState) -> std::fmt::Result {
        match self.id {
            Some(ref id) => writeln!(f, "{:depth$}Argument '{id:?}':", "")?,
            None => writeln!(f, "{:depth$}Argument:", "")?,
        };
        depth.indent();
        self.expression.tree_print(f, depth)
    }
}

#[test]
fn test_argument_debug() {
    let arg1 = Argument {
        id: Some("id1".into()),
        expression: Expression::QualifiedName("my::name1".into()),
        src_ref: SrcRef(None),
    };

    let arg2 = Argument {
        id: None,
        expression: Expression::QualifiedName("my::name2".into()),
        src_ref: SrcRef(None),
    };

    let arg3 = Argument {
        id: Some(Identifier::none()),
        expression: Expression::QualifiedName("my::name2".into()),
        src_ref: SrcRef(None),
    };

    let mut args = ArgumentList::default();

    args.try_push(arg1).expect("test error");
    args.try_push(arg2).expect("test error");
    args.try_push(arg3).expect("test error");

    log::info!("{args}");
    log::info!("{args:?}");
}
