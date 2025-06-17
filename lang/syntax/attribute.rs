// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{src_ref::*, syntax::*};

/// An attribute item.
#[derive(Debug, Clone)]
pub enum Attribute {
    /// A call attribute `foo::bar(baz = 32)`
    NamedTuple(Identifier, ArgumentList),
    /// A name value attribute `foo::baz = baz`
    NameValue(Identifier, Expression),
}

impl Attribute {
    /// Return qualified name for this attribute.
    pub fn identifier(&self) -> &Identifier {
        match self {
            Attribute::NamedTuple(id, _) => id,
            Attribute::NameValue(id, _) => id,
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::NamedTuple(id, argument_list) => {
                writeln!(f, "#[{id}({argument_list})]")
            }
            Attribute::NameValue(id, expression) => {
                writeln!(f, "#[{id} = {expression}]")
            }
        }
    }
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Attribute::NamedTuple(id, argument_list) => SrcRef::merge(id, argument_list),
            Attribute::NameValue(id, expression) => SrcRef::merge(id, expression),
        }
    }
}

/// A list of attributes, e.g. `#foo #[bar, baz = 42]`
#[derive(Debug, Clone, Default)]
pub struct AttributeList(Vec<Attribute>);

impl std::ops::Deref for AttributeList {
    type Target = Vec<Attribute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for AttributeList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|attr| writeln!(f, "{attr}"))
    }
}

impl SrcReferrer for AttributeList {
    fn src_ref(&self) -> SrcRef {
        if self.0.is_empty() {
            SrcRef(None)
        } else {
            SrcRef::merge(
                &self.0.first().expect("One element").src_ref(),
                &self.0.last().expect("Second element").src_ref(),
            )
        }
    }
}
