// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! *Call argument value list* evaluation entity.

use crate::{eval::*, ord_map::*, src_ref::*, value::*};

/// Collection of *call argument values* (e.g. `( x=1, y=2 )`).
///
/// Also provides methods to find a matching call
/// between it and a given *parameter list*.
#[derive(Clone, Debug, Default)]
pub struct CallArgumentValueList(Refer<OrdMap<Identifier, CallArgumentValue>>);

impl CallArgumentValueList {
    /// Create  new call argument value list.
    pub fn new(referrer: &impl SrcReferrer) -> Self {
        Self(Refer::new(OrdMap::default(), referrer.src_ref()))
    }
    /// Create a *call argument value list*.
    ///
    /// Transports code into builtin in `impl` [`Eval`] for [`Call`].
    ///
    /// Shall only be used for builtin symbols.
    /// # Arguments
    pub fn from_code(code: String, referrer: impl SrcReferrer) -> Self {
        let mut value = OrdMap::default();
        value
            .push(CallArgumentValue::new(
                None,
                Value::String(code),
                referrer.src_ref(),
            ))
            .expect("map with one element");
        Self(Refer {
            value,
            src_ref: referrer.src_ref(),
        })
    }

    /// Return a single argument.
    ///
    /// Returns error if there is no or more than one argument available.
    pub fn get_single(&self) -> EvalResult<&CallArgumentValue> {
        if self.len() == 1 {
            if let Some(a) = self.0.first() {
                return Ok(a);
            }
        }

        Err(EvalError::ArgumentCountMismatch {
            args: self.clone(),
            expected: 1,
            found: self.len(),
        })
    }

    /// Fetch an argument by name
    pub fn get_value<'a, T>(&'a self, id: &Identifier) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        let value = &self.0.get(id).expect("no name found").value;
        value.try_into().expect("cannot convert argument value")
    }
}

impl SrcReferrer for CallArgumentValueList {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for CallArgumentValueList {
    type Target = OrdMap<Identifier, CallArgumentValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for CallArgumentValueList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::fmt::Display for CallArgumentValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .value
                .iter()
                .map(|p| p.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
impl From<Vec<CallArgumentValue>> for CallArgumentValueList {
    fn from(value: Vec<CallArgumentValue>) -> Self {
        Self(Refer::none(value.into()))
    }
}
