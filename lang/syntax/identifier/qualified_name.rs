// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Id, eval::*, src_ref::*, syntax::*};

/// A qualifier name consists of a . separated list of identifiers
/// e.g. `a::b::c`
#[derive(Debug, Default, Clone, PartialEq)]
pub struct QualifiedName(pub Vec<Identifier>);

impl SrcReferrer for QualifiedName {
    fn src_ref(&self) -> SrcRef {
        SrcRef::from_vec(&self.0)
    }
}

impl std::ops::Deref for QualifiedName {
    type Target = Vec<Identifier>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for QualifiedName {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for QualifiedName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", join_identifiers(&self.0, "::"))
    }
}

impl From<&str> for QualifiedName {
    fn from(value: &str) -> Self {
        Self(value.split("::").map(Identifier::from).collect())
    }
}

impl From<QualifiedName> for String {
    fn from(value: QualifiedName) -> Self {
        join_identifiers(&value.0, "::")
    }
}

impl PrintSyntax for QualifiedName {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(
            f,
            "{:depth$}QualifiedName: '{}'",
            "",
            join_identifiers(&self.0, "::")
        )
    }
}

impl TryFrom<QualifiedName> for Id {
    type Error = EvalError;

    fn try_from(qualified_name: QualifiedName) -> Result<Self, Self::Error> {
        match qualified_name.as_slice() {
            [identifier] => Ok(identifier.id().clone()),
            _ => Err(EvalError::QualifiedNameIsNoId(qualified_name)),
        }
    }
}
