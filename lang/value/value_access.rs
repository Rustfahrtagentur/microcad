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

    /// Helper function to fetch an argument by string.
    fn by_str<'a, T>(&'a self, id: &str) -> ValueResult<T>
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        let id = &Identifier::no_ref(id);
        if let Some(val) = self.by_id(id) {
            if let Ok(val) = TryInto::try_into(val) {
                Ok(val)
            } else {
                Err(ValueError::CannotConvert(
                    val.to_string(),
                    std::any::type_name::<T>().into(),
                ))
            }
        } else {
            Err(ValueError::IdNotFound(id.clone()))
        }
    }

    /// Fetch an argument value by name as `&str`.
    fn get<'a, T>(&'a self, id: &str) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.by_str(id).expect("No error")
    }

    /// Fetch an argument value by name as `&str`.
    ///
    /// Panics if `id ` cannot be found.`
    fn get_value(&self, id: &str) -> ValueResult<&Value> {
        let id = Identifier::no_ref(id);
        if let Some(value) = self.by_id(&id) {
            Ok(value)
        } else {
            Err(ValueError::IdNotFound(id))
        }
    }

    /// Return `true`, if tuple contains a value with that name
    fn contains_id(&self, id: &Identifier) -> bool {
        self.by_id(id).is_some()
    }
}
