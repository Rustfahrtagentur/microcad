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
        context.scope(
            StackFrame::Module(self.id.clone(), SymbolMap::default()),
            |context| {
                for init in self.init_iter() {
                    if let Ok(multiplicity) =
                        Multiplicity::new(&init.parameters.eval(context)?, args)
                    {
                        log::trace!("Calling init: {}", init.parameters);
                        return multiplicity.call(|symbols| {
                            // when using the
                            let symbols = if init.implicit {
                                log::trace!("Symbols:\n{symbols}");
                                context.set_module_symbols(symbols)?;
                                log::trace!("Context:\n{context}");
                                SymbolMap::new()
                            } else {
                                symbols
                            };
                            self.body.eval_to_node(symbols, context).map(|x| x.into())
                        });
                    }
                }
                context.error(
                    args,
                    EvalError::NoMatchingInit(self.id.clone(), args.clone()),
                )?;
                Ok(Value::None)
            },
        )
    }
}
