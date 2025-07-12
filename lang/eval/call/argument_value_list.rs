// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Argument value list* evaluation entity.

use crate::{eval::*, src_ref::*, value::*};

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default)]
pub struct ArgumentValueList {
    map: std::collections::HashMap<Identifier, ArgumentValue>,
    src_ref: SrcRef,
}

impl ArgumentValueList {
    /// Create a *argument value list*.
    ///
    /// Transports code into builtin in `impl` [`Eval`] for [`Call`].
    ///
    /// Shall only be used for builtin symbols.
    /// # Arguments
    pub fn from_code(code: String, referrer: impl SrcReferrer) -> Self {
        let map: std::collections::HashMap<Identifier, ArgumentValue> = [(
            Identifier::none(),
            (ArgumentValue::new(Value::String(code), referrer.src_ref())),
        )]
        .into_iter()
        .collect();
        Self {
            map,
            src_ref: referrer.src_ref(),
        }
    }

    /// Return a single argument.
    ///
    /// Returns error if there is no or more than one argument available.
    pub fn get_single(&self) -> EvalResult<(&Identifier, &ArgumentValue)> {
        if self.map.len() == 1 {
            if let Some(a) = self.map.iter().next() {
                return Ok(a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.clone(),
            expected: 1,
            found: self.map.len(),
        })
    }

    /// Get value by type
    pub fn get_by_type(&self, ty: &Type) -> Option<(&Identifier, &ArgumentValue)> {
        self.map.iter().find(|(_, arg)| arg.value.ty() == *ty)
    }
}

impl ValueAccess for ArgumentValueList {
    fn by_id(&self, id: &Identifier) -> Option<&Value> {
        self.map.get(id).map(|arg| &arg.value)
    }

    fn by_ty(&self, ty: &Type) -> Option<&Value> {
        self.get_by_type(ty).map(|(_, arg)| &arg.value)
    }
}

impl std::ops::Deref for ArgumentValueList {
    type Target = std::collections::HashMap<Identifier, ArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl SrcReferrer for ArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let mut v = self
                .map
                .iter()
                .map(|(id, p)| format!("{id}: {p}"))
                .collect::<Vec<_>>();
            v.sort();
            v.join(", ")
        })
    }
}

impl FromIterator<(Identifier, ArgumentValue)> for ArgumentValueList {
    fn from_iter<T: IntoIterator<Item = (Identifier, ArgumentValue)>>(iter: T) -> Self {
        let map: std::collections::HashMap<_, _> = iter.into_iter().collect();
        Self {
            src_ref: SrcRef::merge_all(map.values().map(|a| a.src_ref())),
            map,
        }
    }
}
