// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value access trait

use crate::{syntax::*, value::*};

/// Trait for Value Lists
pub trait ValueAccess {
    /// Find named value by identifier.
    fn by_id(&self, id: &Identifier) -> Option<&Value>;

    /// Find unnamed value by type.
    fn by_ty(&self, ty: &Type) -> Option<&Value>;

    /// Fetch an argument value by name as `&str`.
    ///
    /// Panics if `id ` cannot be found.`
    fn get<'a, T>(&'a self, id: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.by_id(&Identifier::no_ref(id))
            .map(|t| TryInto::try_into(t).expect("Value"))
            .expect("some value")
    }

    /// Fetch an argument value by name as `&str`.
    ///
    /// Panics if `id ` cannot be found.`
    fn get_value(&self, id: &str) -> &Value {
        self.by_id(&Identifier::no_ref(id)).expect("")
    }
}
