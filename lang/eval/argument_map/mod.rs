// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Argument map

mod argument_match;
mod multi_argument_map;
mod multiplicity;

pub use argument_match::*;
pub use multi_argument_map::*;
pub use multiplicity::*;

use crate::{eval::*, src_ref::*, value::*};

/// Map of named call arguments
#[derive(Clone, Debug, Default)]
pub struct ArgumentMap(Refer<std::collections::HashMap<Identifier, Value>>);

impl ArgumentMap {
    /// Create empty argument map
    pub fn new(src_ref: SrcRef) -> Self {
        Self(Refer::new(std::collections::HashMap::new(), src_ref))
    }

    /// Fetch an argument by name
    pub fn get_value<'a, T>(&'a self, id: &Identifier) -> T
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.0
            .get(id)
            .expect("no name found")
            .try_into()
            .expect("cannot convert argument value")
    }
}

impl SrcReferrer for ArgumentMap {
    fn src_ref(&self) -> SrcRef {
        self.0.src_ref()
    }
}

impl std::ops::Deref for ArgumentMap {
    type Target = std::collections::HashMap<Identifier, Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ArgumentMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ArgumentMatch for ArgumentMap {
    fn insert_and_remove_from_parameters(
        &mut self,
        value: Value,
        parameter_value: &ParameterValue,
        parameter_values: &mut ParameterValueList,
    ) -> EvalResult<TypeCheckResult> {
        let id = &parameter_value.id;
        parameter_values.remove(id);
        self.insert(id.clone(), value.clone());
        Ok(TypeCheckResult::SingleMatch)
    }
}

#[test]
fn argument_match_single() {
    let parameter_values = ParameterValueList::new(vec![crate::parameter_value!(a: Scalar)]);

    let call_argument_value_list =
        CallArgumentValueList::from(vec![crate::call_argument_value!(a: Scalar = 5.0)]);

    let arg_map =
        ArgumentMap::find_match(&call_argument_value_list, &parameter_values).expect("Valid match");

    let a = arg_map.get(&Identifier::no_ref("a"));
    assert!(a.is_some());
    let a = a.expect("internal test error");
    assert!(a == &Value::Scalar(Refer::none(5.0)));
}
