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
}

// TODO impl FromIterator instead
impl<T> From<std::slice::Iter<'_, (&'static str, T)>> for Tuple
where
    T: Into<Value> + Clone + std::fmt::Debug,
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

impl TryFrom<&Tuple> for Color {
    type Error = ValueError;

    fn try_from(tuple: &Tuple) -> Result<Self, Self::Error> {
        let (r, g, b, a) = (
            tuple.by_id(&Identifier::no_ref("r")),
            tuple.by_id(&Identifier::no_ref("g")),
            tuple.by_id(&Identifier::no_ref("b")),
            tuple
                .by_id(&Identifier::no_ref("a"))
                .unwrap_or(&Value::Quantity(Quantity::new(1.0, QuantityType::Scalar)))
                .clone(),
        );

        match (r, g, b, a) {
            (
                Some(Value::Quantity(Quantity {
                    value: r,
                    quantity_type: QuantityType::Scalar,
                })),
                Some(Value::Quantity(Quantity {
                    value: g,
                    quantity_type: QuantityType::Scalar,
                })),
                Some(Value::Quantity(Quantity {
                    value: b,
                    quantity_type: QuantityType::Scalar,
                })),
                Value::Quantity(Quantity {
                    value: a,
                    quantity_type: QuantityType::Scalar,
                }),
            ) => Ok(Color::new(*r as f32, *g as f32, *b as f32, a as f32)),
            _ => Err(ValueError::CannotConvertToColor(Box::new(tuple.clone()))),
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
                .map(|(id, v)| format!("{id}: {v}"))
                .chain(self.unnamed.values().map(|v| format!("{v}")))
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
    let tuple1: Tuple = [
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Volume,
            }),
        ),
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Length,
            }),
        ),
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Area,
            }),
        ),
    ]
    .iter()
    .into();
    let tuple2: Tuple = [
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Length,
            }),
        ),
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Area,
            }),
        ),
        (
            "",
            Value::Quantity(Quantity {
                value: 1.0,
                quantity_type: QuantityType::Volume,
            }),
        ),
    ]
    .iter()
    .into();

    assert_eq!(tuple1, tuple2);
}
