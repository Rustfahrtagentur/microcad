use crate::{eval::*, syntax::*};

impl CallTrait for FunctionDefinition {
    fn call(&self, args: &CallArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        log::trace!("Calling function '{}'", self.id);
        match Multiplicity::new(&self.signature.parameters.eval(context)?, args) {
            Ok(multiplicity) => {
                multiplicity.call(|symbols| self.body.eval_with_locals(symbols, context))
            }
            Err(err) => {
                context.error(args, err)?;
                Ok(Value::None)
            }
        }
    }
}

#[test]
fn fn_multiplicity() {
    let mut context = Context::from_str(
        r#"

// function f with all different parameters
function f(a: Scalar, b: Length, c: Area) {}
 
// all named
f(a=0, b=1m, c=2m²);
 
// some named
f(a=0, b=1m, 2m²);
 
// all named, alternative order: c,b,a
f(2m², b=1m, a=0);
 
// one named
f(0, b=1m, 2m²);
 
// none named, alternative order:  c,a,b
f(2m², 0, 1m);
 
// function g with all different parameters but a and d
function g(a: Scalar, b: Length, c: Area, d: Scalar) {}
 
// all named
g(a=0, b=1m, c=2m², d=3);
 
// some named
g(a=1, 2cm², 1m, d=3);
 
// only a and d named, alternative order: d,b,c,a
g(d=3, 1m, 2cm², a=0);

    "#,
        no_builtin_namespace(),
        &[],
    )
    .expect("load test ok");

    context.eval().expect("eval ok");

    assert!(!context.has_errors());
}
