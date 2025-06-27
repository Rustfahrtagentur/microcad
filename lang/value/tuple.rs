// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use crate::{ty::*, value::*};

/// Tuple with named values
///
/// Names are optional, which means Identifiers can be empty.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Tuple {
    pub(crate) named: std::collections::HashMap<Identifier, Value>,
    pub(crate) unnamed: std::collections::HashMap<Type, Value>,
}

impl Tuple {
    /// Find named value by identifier.
    pub fn by_id(&self, id: &Identifier) -> Option<&Value> {
        self.named.get(id)
    }

    /// Find unnamed value by type.
    pub fn by_ty(&self, ty: &Type) -> Option<&Value> {
        self.unnamed.get(ty)
    }
}

// TODO impl FromIterator instead
impl<T: Into<Value> + Clone> From<std::slice::Iter<'_, (&'static str, T)>> for Tuple {
    fn from(values: std::slice::Iter<'_, (&'static str, T)>) -> Self {
        let (unnamed, named) = values
            .map(|(k, v)| (Identifier::no_ref(k), (*v).clone().into()))
            .partition(|(k, _)| k.is_empty());
        Self {
            named,
            unnamed: unnamed.into_values().map(|v| (v.ty(), v)).collect(),
        }
    }
}

impl From<Vec2> for Tuple {
    fn from(v: Vec2) -> Self {
        [("x", v.x), ("y", v.y)].iter().into()
    }
}

impl From<Vec3> for Tuple {
    fn from(v: Vec3) -> Self {
        [("x", v.x), ("y", v.y), ("z", v.z)].iter().into()
    }
}

impl From<Color> for Tuple {
    fn from(color: Color) -> Self {
        [
            ("r", color.r),
            ("g", color.g),
            ("b", color.b),
            ("a", color.a),
        ]
        .iter()
        .into()
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = self
                .named
                .iter()
                .map(|(id, v)| format!("{id} : {t}={v}", t = v.ty()))
                .chain(self.unnamed.iter().map(|(ty, v)| format!("{v}: {ty}")))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl crate::ty::Ty for Tuple {
    fn ty(&self) -> Type {
        Type::Tuple(
            TupleType {
                named: self
                    .named
                    .iter()
                    .map(|(id, v)| (id.clone(), v.ty()))
                    .collect(),
                unnamed: self.unnamed.values().map(|v| v.ty()).collect(),
            }
            .into(),
        )
    }
}
