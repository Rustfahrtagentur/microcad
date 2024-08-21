mod builtin_module;
mod call;
mod context;
mod errors;
mod parameter;
mod symbols;

pub use builtin_module::*;
pub use call::*;
pub use context::*;
pub use errors::*;
pub use parameter::*;
pub use symbols::*;

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output, EvalError>;
}
