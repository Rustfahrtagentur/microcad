// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Argument value list* evaluation entity.

use crate::{
    eval::*,
    src_ref::{self, *},
    value::*,
};
use derive_more::Deref;

/// Collection of *argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default, Deref)]
pub struct ArgumentValueList {
    #[deref]
    map: Vec<(Identifier, ArgumentValue)>,
    src_ref: SrcRef,
}

impl ArgumentValueList {
    /// Create new [`ArgumentValueList`]
    pub fn new(map: Vec<(Identifier, ArgumentValue)>, src_ref: SrcRef) -> Self {
        Self { map, src_ref }
    }

    /// Create a *argument value list*.
    ///
    /// Transports code into builtin in `impl` [`Eval`] for [`Call`].
    ///
    /// Shall only be used for builtin symbols.
    /// # Arguments
    pub fn from_code(code: String, referrer: impl SrcReferrer) -> Self {
        let map = [(
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
            if let Some(a) = self.map.first() {
                return Ok((&a.0, &a.1));
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
        let arg = self.map.iter().find(|(_, arg)| arg.value.ty() == *ty);
        arg.map(|arg| (&arg.0, &arg.1))
    }
}

impl ValueAccess for ArgumentValueList {
    fn by_id(&self, id: &Identifier) -> Option<&Value> {
        self.map
            .iter()
            .find(|(i, _)| i == id)
            .map(|arg| &arg.1.value)
    }

    fn by_ty(&self, ty: &Type) -> Option<&Value> {
        self.get_by_type(ty).map(|(_, arg)| &arg.value)
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
        let map: Vec<_> = iter.into_iter().collect();
        Self {
            src_ref: SrcRef::merge_all(map.iter().map(|(_, v)| v.src_ref())),
            map,
        }
    }
}
