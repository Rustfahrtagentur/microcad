// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module definition syntax element evaluation

use crate::{eval::*, syntax::*, value::*};

impl CallTrait for ModuleDefinition {
    /// Evaluate the call of a module
    ///
    /// The evaluation considers multiplicity, which means that multiple nodes maybe created.
    ///
    /// Example:
    /// Consider the `module a(b: Scalar) { }`.
    /// Calling the module `a([1.0, 2.0])` results in two nodes with `b = 1.0` and `b = 2.0`, respectively.
    fn call(&self, args: &CallArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        for init in self.init_iter() {
            log::trace!("Calling function '{}'", self.id);
            match Multiplicity::new(&init.parameters.eval(context)?, args) {
                Ok(multiplicity) => {
                    multiplicity.call(|symbols| {
                        self.body.eval_to_node(symbols, context).map(|x| x.into())
                    })?;
                }
                Err(err) => {
                    context.error(args, err)?;
                    return Ok(Value::None);
                }
            }
        }

        Err(EvalError::NoMatchingInit(self.id.clone(), args.clone()))
    }
}
