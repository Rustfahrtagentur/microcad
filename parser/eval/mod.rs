//! evaluation of parsed content

mod builtin_function;
mod builtin_module;
mod call;
mod context;
mod errors;
mod parameter;
mod symbols;
mod ty;
mod value;

pub use builtin_function::*;
pub use builtin_module::*;
pub use call::*;
pub use context::*;
pub use errors::*;
pub use parameter::*;
pub use symbols::*;
pub use ty::*;
pub use value::*;

pub type Result<T> = std::result::Result<T, EvalError>;

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output>;
}
