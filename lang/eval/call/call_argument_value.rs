use crate::{eval::*, ord_map::*, src_ref::*};

#[derive(Clone, Debug)]
pub struct CallArgumentValue {
    pub name: Option<Id>,
    pub value: Value,
    src_ref: SrcRef,
}

impl OrdMapValue<Id> for CallArgumentValue {
    fn key(&self) -> Option<Id> {
        self.name.clone()
    }
}

impl SrcReferrer for CallArgumentValue {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl CallArgumentValue {
    pub fn new(name: Option<Id>, value: Value, src_ref: SrcRef) -> Self {
        Self {
            name,
            value,
            src_ref,
        }
    }
}

#[macro_export]
macro_rules! call_argument_value {
    ($name:ident: $ty:ident = $value:expr) => {
        CallArgumentValue::new(
            Some(stringify!($name).into()),
            Value::$ty($crate::src_ref::Refer::none($value)),
            $crate::src_ref::SrcRef(None),
        )
    };
    ($ty:ident = $value:expr) => {
        CallArgumentValue::new(
            None,
            Value::$ty($crate::src_ref::Refer::none($value)),
            $crate::src_ref::SrcRef(None),
        )
    };
    () => {};
}
