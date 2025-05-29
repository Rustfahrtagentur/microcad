// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{src_ref::SrcReferrer, syntax::*};

/// An attribute item.
pub enum Attribute {
    /// A tag attribute `foo::bar`
    Tag(QualifiedName),
    /// A call attribute `foo::bar(baz = 32)`
    Call(Call),
    /// A name value attribute `foo::baz = baz`
    NameValue(QualifiedName, Expression),
}

impl Attribute {
    /// Return qualified name for this attribute.
    pub fn qualified_name(&self) -> &QualifiedName {
        match self {
            Attribute::Tag(qualified_name) => qualified_name,
            Attribute::Call(call) => &call.name,
            Attribute::NameValue(qualified_name, _) => qualified_name,
        }
    }
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Attribute::Tag(qualified_name) => qualified_name.src_ref(),
            Attribute::Call(call) => call.src_ref(),
            Attribute::NameValue(qualified_name, _expression) => {
                // FIXME: This should return a merged src_ref of `qualified_name` and `expression`
                qualified_name.src_ref()
            }
        }
    }
}

/// A list of attributes, e.g. `#foo #[bar, baz = 42]`
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
