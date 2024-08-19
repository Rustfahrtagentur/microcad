mod context;
mod errors;
mod symbols;

pub use context::*;
pub use errors::*;
pub use symbols::*;

pub trait Eval {
    type Output;

    /// Evaluate the type into an expression
    fn eval(&self, context: &mut Context) -> Result<Self::Output, Error>;
}
