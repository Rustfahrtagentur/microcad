use super::{SrcRef, SrcReferrer};
#[derive(Clone, Default, Ord, Debug, PartialEq, PartialOrd)]
pub struct Refer<T> {
    pub value: T,
    pub src_ref: SrcRef,
}

impl<T> Refer<T> {
    pub fn none(value: T) -> Self {
        Self {
            value,
            src_ref: SrcRef(None),
        }
    }
    pub fn new(value: T, src_ref: SrcRef) -> Self {
        Self { value, src_ref }
    }
    pub fn merge<U, V>(left: Refer<U>, right: Refer<V>, f: fn(U, V) -> T) -> Self {
        Self {
            value: f(left.value, right.value),
            src_ref: SrcRef::merge(left.src_ref, right.src_ref),
        }
    }
    pub fn map<U>(self, f: fn(T) -> U) -> Refer<U> {
        Refer::<U> {
            value: f(self.value),
            src_ref: self.src_ref,
        }
    }
}

impl<T> std::ops::Deref for Refer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> std::ops::DerefMut for Refer<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}

impl<T> SrcReferrer for Refer<T> {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl<T: Eq> Eq for Refer<T> {}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Refer<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            value: self.value.neg(),
            src_ref: self.src_ref,
        }
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add for Refer<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value.add(other.value),
            src_ref: SrcRef::merge(self.src_ref, other.src_ref),
        }
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Refer<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            value: self.value.sub(other.value),
            src_ref: SrcRef::merge(self.src_ref, other.src_ref),
        }
    }
}

impl<T: std::ops::Mul<Output = T>> std::ops::Mul for Refer<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            value: self.value.mul(other.value),
            src_ref: SrcRef::merge(self.src_ref, other.src_ref),
        }
    }
}

impl<T: std::ops::Div<Output = T>> std::ops::Div for Refer<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            value: self.value.div(other.value),
            src_ref: SrcRef::merge(self.src_ref, other.src_ref),
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Refer<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Refer<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}
