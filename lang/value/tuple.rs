// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Named tuple evaluation entity

use std::collections::HashMap;

use crate::{src_ref::*, ty::*, value::*};

/// Tuple with named values
///
/// Names are optional, which means Identifiers can be empty.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Tuple {
    pub(crate) named: std::collections::HashMap<Identifier, Value>,
    pub(crate) unnamed: std::collections::HashMap<Type, Value>,
    pub(crate) src_ref: SrcRef,
}

/// Create Tuple from µcad code for tests
#[cfg(test)]
#[macro_export]
macro_rules! tuple {
    ($code:expr) => {{
        use $crate::eval::*;
        match $crate::tuple_expression!($code)
            .eval(&mut Default::default())
            .expect("evaluation error")
        {
            Value::Tuple(tuple) => *tuple,
            _ => panic!(),
        }
    }};
}

/// Create a Value::Tuple from items
#[macro_export]
macro_rules! create_tuple_value {
    ($($key:ident = $value:expr),*) => {
        Value::Tuple(Box::new($crate::create_tuple!($( $key = $value ),*)))
    };
}

/// Create a Tuple from items
#[macro_export]
macro_rules! create_tuple {
        ($($key:ident = $value:expr),*) => {
                [$( (stringify!($key), $value) ),* ]
                    .iter()
                    .into()
    };
}

impl Tuple {
    /// Create new named tuple.
    pub fn new_named(named: std::collections::HashMap<Identifier, Value>, src_ref: SrcRef) -> Self {
        Self {
            named,
            unnamed: HashMap::default(),
            src_ref,
        }
    }

    /// Insert new (or overwrite existing) value into tuple
    pub fn insert(&mut self, id: Identifier, value: Value) {
        if id.is_empty() {
            self.unnamed.insert(value.ty(), value);
        } else {
            self.named.insert(id, value);
        }
    }

    /// Return an iterator over all named values
    pub fn named_iter(&self) -> std::collections::hash_map::Iter<'_, Identifier, Value> {
        if !self.unnamed.is_empty() {
            log::warn!("using named_iter() on a tuple which has unnamed items too")
        }
        self.named.iter()
    }

    /// Return the tuple type.
    pub fn tuple_type(&self) -> TupleType {
        TupleType {
            named: self
                .named
                .iter()
                .map(|(id, v)| (id.clone(), v.ty()))
                .collect(),
            unnamed: self.unnamed.values().map(|v| v.ty()).collect(),
        }
    }

    /// Dissolve unnamed them.
    ///
    /// Transparent tuples are unnamed tuple items of a tuple.
    ///
    /// ```,µcad
    /// assert_eq!( (x=0, (y=0, z=0)), (x=0, y=0, z=0) );
    /// ///               ^ unnamed tuple
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
    pub fn multiplicity<P: FnMut(Tuple)>(&self, mut ids: Vec<Identifier>, mut p: P) {
        log::trace!("combining: {ids:?}:");
        // count array indexes for items which shall be multiplied and number of overall combinations
        let mut combinations = 1;
        ids.sort();
        let mut counts: HashMap<Identifier, (_, _)> = ids
            .into_iter()
            .map(|id| {
                let counter = if let Some(Value::Array(array)) = &self.named.get(&id) {
                    let len = array.len();
                    combinations *= len;
                    (0, len)
                } else {
                    panic!("'{id}' found in tuple but no list:\n{self:#?}");
                };
                (id, counter)
            })
            .collect();

        log::trace!("multiplicity: {combinations} combinations:");

        // call predicate for each version of the tuple
        for _ in 0..combinations {
            let mut counted = false;
            let mut named: Vec<_> = self.named.iter().collect();
            named.sort_by(|lhs, rhs| lhs.0.cmp(rhs.0));
            let tuple = named
                .into_iter()
                .map(|(id, v)| match v {
                    Value::Array(array) => {
                        if let Some((count, len)) = counts.get_mut(id) {
                            let item = (
                                id.clone(),
                                array.get(*count).expect("array index not found").clone(),
                            );
                            if !counted {
                                *count += 1;
                                if *count == *len {
                                    *count = 0
                                } else {
                                    counted = true;
                                }
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

impl ValueAccess for Tuple {
    fn by_id(&self, id: &Identifier) -> Option<&Value> {
        self.named.get(id)
    }

    fn by_ty(&self, ty: &Type) -> Option<&Value> {
        self.unnamed.get(ty)
    }
}

impl SrcReferrer for Tuple {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
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
            src_ref: SrcRef(None),
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
            src_ref: SrcRef::merge_all(
                named
                    .iter()
                    .map(|(id, _)| id.src_ref())
                    .chain(unnamed.iter().map(|(id, _)| id.src_ref())),
            ),
            named: named.into_iter().collect(),
            unnamed: unnamed.into_iter().map(|(_, v)| (v.ty(), v)).collect(),
        }
    }
}

impl From<Vec2> for Tuple {
    fn from(v: Vec2) -> Self {
        create_tuple!(x = v.x, y = v.y)
    }
}

impl From<Vec3> for Tuple {
    fn from(v: Vec3) -> Self {
        create_tuple!(x = v.x, y = v.y, z = v.z)
    }
}

impl From<Color> for Tuple {
    fn from(color: Color) -> Self {
        create_tuple!(r = color.r, g = color.g, b = color.b, a = color.a)
    }
}

impl From<Size2D> for Tuple {
    fn from(size: Size2D) -> Self {
        create_tuple!(
            width = Value::from(Quantity::length(size.width)),
            height = Value::from(Quantity::length(size.height))
        )
    }
}

impl From<Tuple> for Value {
    fn from(tuple: Tuple) -> Self {
        Value::Tuple(Box::new(tuple))
    }
}

impl FromIterator<Tuple> for Tuple {
    fn from_iter<T: IntoIterator<Item = Tuple>>(iter: T) -> Self {
        let tuples: Vec<_> = iter.into_iter().collect();
        Self {
            src_ref: SrcRef::merge_all(tuples.iter().map(|t| t.src_ref())),
            named: Default::default(),
            unnamed: tuples
                .into_iter()
                .map(|t| (Type::Tuple(t.tuple_type().into()), Value::Tuple(t.into())))
                .collect(),
        }
    }
}

impl IntoIterator for Tuple {
    type Item = (Identifier, Value);
    type IntoIter = std::collections::hash_map::IntoIter<Identifier, Value>;

    fn into_iter(self) -> Self::IntoIter {
        if !self.unnamed.is_empty() {
            log::warn!("trying to iterate Tuple with unnamed items");
        }
        self.named.into_iter()
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

impl TryFrom<&Tuple> for Size2D {
    type Error = ValueError;

    fn try_from(tuple: &Tuple) -> Result<Self, Self::Error> {
        let (width, height) = (
            tuple.by_id(&Identifier::no_ref("width")),
            tuple.by_id(&Identifier::no_ref("height")),
        );

        match (width, height) {
            (
                Some(Value::Quantity(Quantity {
                    value: width,
                    quantity_type: QuantityType::Length,
                })),
                Some(Value::Quantity(Quantity {
                    value: height,
                    quantity_type: QuantityType::Length,
                })),
            ) => Ok(Size2D {
                width: *width,
                height: *height,
            }),
            _ => Err(ValueError::CannotConvert(
                tuple.clone().into(),
                "Size2D".into(),
            )),
        }
    }
}

impl std::fmt::Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "({items})",
            items = {
                let mut items = self
                    .named
                    .iter()
                    .map(|(id, v)| format!("{id}: {v}"))
                    .chain(self.unnamed.values().map(|v| format!("{v}")))
                    .collect::<Vec<String>>();
                items.sort();
                items.join(", ")
            }
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
        tuple!("(v=1.0m³, l=1.0m, a=1.0m²)"),
        tuple!("(l=1.0m, a=1.0m², v=1.0m³)")
    );
}

#[test]
fn tuple_not_equal() {
    assert_ne!(
        tuple!("(d=1.0g/mm³, l=1.0m, a=1.0m²)"),
        tuple!("(l=1.0m, a=1.0m², v=1.0m³)")
    );
    assert_ne!(
        tuple!("(l=1.0m, a=1.0m²)"),
        tuple!("(l=1.0m, a=1.0m², v=1.0m³)")
    );
}

#[test]
fn multiplicity_check() {
    let tuple = tuple!("(x = [1, 2, 3], y = [1, 2], z = 1)");

    let ids: Vec<Identifier> = ["x".into(), "y".into()].into_iter().collect();
    tuple.multiplicity(ids, |tuple| println!("{tuple}"));
}
