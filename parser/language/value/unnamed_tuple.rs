use super::{Value, ValueError, ValueList};
use crate::language::lang_type::{Ty, Type, UnnamedTupleType};

#[derive(Clone, Debug, PartialEq)]
pub struct UnnamedTuple(ValueList);

impl UnnamedTuple {
    pub fn binary_op(
        self,
        rhs: Self,
        op: char,
        f: impl Fn(Value, Value) -> Result<Value, ValueError>,
    ) -> Result<Self, ValueError> {
        if self.0.len() != rhs.0.len() {
            return Err(ValueError::TupleLengthMismatchForOperator {
                operator: op,
                lhs: self.0.len(),
                rhs: rhs.0.len(),
            });
        }
        let mut result = ValueList::new();
        for (l, r) in self.0.iter().zip(rhs.0.iter()) {
            let add_result = f(l.clone(), r.clone())?;
            result.push(add_result);
        }
        Ok(UnnamedTuple(result))
    }
}

impl From<ValueList> for UnnamedTuple {
    fn from(value: ValueList) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for UnnamedTuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl Ty for UnnamedTuple {
    fn ty(&self) -> Type {
        Type::UnnamedTuple(UnnamedTupleType(
            self.0.iter().map(|v| v.ty().clone()).collect(),
        ))
    }
}

impl std::ops::Add for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn add(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '+', |lhs, rhs| lhs + rhs)
    }
}

impl std::ops::Sub for UnnamedTuple {
    type Output = Result<UnnamedTuple, ValueError>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, '-', |lhs, rhs| lhs - rhs)
    }
}
