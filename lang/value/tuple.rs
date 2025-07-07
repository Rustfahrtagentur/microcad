// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use std::collections::HashMap;

use crate::{ty::*, value::*};

/// Tuple with named values
///
/// Names are optional, which means Identifiers can be empty.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Tuple {
    pub(crate) named: std::collections::HashMap<Identifier, Value>,
    pub(crate) unnamed: std::collections::HashMap<Type, Value>,
}

/// Create a Tuple from items
#[macro_export]
macro_rules! tuple {
        ($($key:ident = $value:expr),*) => {
                [$( (stringify!($key), $value) ),* ]
                    .iter()
                    .into()
    };
}

/// Create a Tuple from Quantity items
#[macro_export]
macro_rules! tuple_quantity {
        ($($quantity:ident = $value:expr),*) => {
            Into::<$crate::value::Tuple>::into([$((
                "",
                $crate::value::Value::Quantity($crate::value::Quantity {
                    value: $value,
                    quantity_type: $crate::ty::QuantityType::$quantity,
                }),
            )),*]
            .iter())
        };
}

impl Tuple {
    /// Create new named tuple.
    pub fn new_named(named: std::collections::HashMap<Identifier, Value>) -> Self {
        Self {
            named,
            unnamed: HashMap::default(),
        }
    }

    /// Find named value by identifier.
    pub fn by_id(&self, id: &Identifier) -> Option<&Value> {
        self.named.get(id)
    }

    /// Find unnamed value by type.
    pub fn by_ty(&self, ty: &Type) -> Option<&Value> {
        self.unnamed.get(ty)
    }

    /// Fetch an argument by name as `&str`.
    ///
    /// This method does not provide error handling and is supposed to be used for built-ins.
    pub fn get<'a, T>(&'a self, id: &str) -> Option<T>
    where
        T: std::convert::TryFrom<&'a Value>,
        T::Error: std::fmt::Debug,
    {
        self.named
            .get(&Identifier::no_ref(id))
            .map(|value| TryInto::try_into(value).expect("Value"))
    }

    /// Dissolve unnamed them.
    ///
    /// Transparent tuples are unnamed tuple items of a tuple.
    ///
    /// ```,µcad
    /// assert_eq!( (x=0, (y=0, z=0)), (x=0, y=0, z=0) );
    /// ///               ^ unnamed tuple tem
    pub fn ray(&mut self) {
        self.unnamed.retain(|_, value| {
            if let Value::Tuple(tuple) = value {
                tuple.ray();
                tuple.named.drain().for_each(|(k, v)| {
                    self.named.insert(k, v);
                });
                false
            } else {
                true
            }
        });
    }

    /// Call a predicate for each tuple multiplicity.
    ///
    /// - `ids`: Items to multiply.
    /// - `p`: Predicate to call for each resulting tuple.
    ///
    /// # Example
    ///
    /// | Input           | Predicate's Parameters |
    /// |-----------------|------------------------|
    /// | `([x₀, x₁], y)` | `(x₀, y)`, `(x₁, y)`   |
    ///
    pub fn multiplicity<P: Fn(Tuple)>(&self, ids: std::collections::HashSet<Identifier>, p: P) {
        // count array indexes for items which shall be multiplied and number of overall combinations
        let mut combinations = 1;
        let mut counts: HashMap<Identifier, (_, _)> = ids
            .into_iter()
            .map(|id| {
                let len = if let Value::Array(array) = &self.named[&id] {
                    let len = array.len();
                    combinations *= len;
                    (len, len)
                } else {
                    panic!("'{id}' found in tuple but no list");
                };
                (id, len)
            })
            .collect();

        // call predicate for each version of the tuple
        while combinations > 0 {
            let mut counted = false;
            let tuple = self
                .named
                .iter()
                .map(|(id, v)| match v {
                    Value::Array(array) => {
                        if let Some((count, len)) = counts.get_mut(id) {
                            let item = (
                                id.clone(),
                                array.get(*count).expect("array index not found").clone(),
                            );
                            if !counted {
                                if *count == 0 {
                                    *count = *len
                                } else {
                                    *count -= 1
                                }
                                combinations -= 1;
                                counted = false;
                            }
                            item
                        } else {
                            panic!("'{id}' found in tuple but no list");
                        }
                    }
                    _ => (id.clone(), v.clone()),
                })
                .collect();
            p(tuple);
        }
    }
}

// TODO impl FromIterator instead
impl<T> From<std::slice::Iter<'_, (&'static str, T)>> for Tuple
where
    T: Into<Value> + Clone,
{
    fn from(iter: std::slice::Iter<'_, (&'static str, T)>) -> Self {
        let (unnamed, named): (Vec<_>, _) = iter
            .map(|(k, v)| (Identifier::no_ref(k), (*v).clone().into()))
            .partition(|(k, _)| k.is_empty());
        Self {
            named: named.into_iter().collect(),
            unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
        }
    }
}

impl FromIterator<(Identifier, Value)> for Tuple {
    fn from_iter<T: IntoIterator<Item = (Identifier, Value)>>(iter: T) -> Self {
        let (unnamed, named): (Vec<_>, _) = iter
            .into_iter()
            .map(|(k, v)| (k, v.clone()))
            .partition(|(k, _)| k.is_empty());
        Self {
            named: named.into_iter().collect(),
            unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
        }
    }
}

impl From<Vec2> for Tuple {
    fn from(v: Vec2) -> Self {
        tuple!(x = v.x, y = v.y)
    }
}

impl From<Vec3> for Tuple {
    fn from(v: Vec3) -> Self {
        tuple!(x = v.x, y = v.y, z = v.z)
    }
}

impl From<Color> for Tuple {
    fn from(color: Color) -> Self {
        tuple!(r = color.r, g = color.g, b = color.b, a = color.a)
    }
}

impl From<Tuple> for Value {
    fn from(tuple: Tuple) -> Self {
        Value::Tuple(Box::new(tuple))
    }
}

impl<'a> TryFrom<&'a Value> for &'a Tuple {
    type Error = ValueError;

    fn try_from(value: &'a Value) -> Result<Self, Self::Error> {
        match value {
            Value::Tuple(tuple) => Ok(tuple),
            _ => Err(ValueError::CannotConvert(
                value.clone(),
                "Tuple".to_string(),
            )),
        }
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

#[test]
fn tuple_equal() {
    assert_eq!(
        tuple_quantity!(Volume = 1.0, Length = 1.0, Area = 1.0),
        tuple_quantity!(Length = 1.0, Area = 1.0, Volume = 1.0)
    );
}

#[test]
fn tuple_not_equal() {
    assert_ne!(
        tuple_quantity!(Density = 1.0, Length = 1.0, Area = 1.0),
        tuple_quantity!(Length = 1.0, Area = 1.0, Volume = 1.0)
    );
    assert_ne!(
        tuple_quantity!(Length = 1.0, Area = 1.0),
        tuple_quantity!(Length = 1.0, Area = 1.0, Volume = 1.0)
    );
}
