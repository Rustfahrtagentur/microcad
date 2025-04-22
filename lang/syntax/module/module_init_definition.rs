// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module initialization definition syntax element

use crate::{diag::PushDiag, eval::{ArgumentMap, EvalContext, EvalError, EvalResult}, objects::{ObjectNode, ObjectProperties}, src_ref::*, syntax::*, value::Value};

/// Module initialization definition
///
/// Example:
///
/// ```uCAD
/// module a(a: Length) {
///     init(b: Length) { a = 2.0*b; } // The init definition
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ModuleInitDefinition {
    /// Parameter list for this init definition
    pub parameters: ParameterList,
    /// Body if the init definition
    pub body: Body,
    /// Source reference
    pub src_ref: SrcRef,
}


impl ModuleInitDefinition {
    /// Evaluate a call to the module init definition
    pub fn eval_to_node(&self, args: &ArgumentMap, mut props: ObjectProperties, context: &mut EvalContext) -> EvalResult<ObjectNode> {
        context.open_scope();

        // Add values from argument map as local values
        for (id, value) in args.iter() {
            props.assign_and_add_local_value(id, value.clone(), context);
        }

        use crate::eval::Eval;

        let mut nodes = Vec::new();
        for statement in &self.body.statements {
            match statement {
                Statement::Assignment(assignment) => {
                    let id = assignment.name.id();
                    let value = assignment.value.eval(context)?;

                    props.assign_and_add_local_value(id, value, context);
                }
                Statement::Expression(expression) => {
                    nodes.append(&mut expression.eval(context)?.fetch_nodes())
                }
                _ => {
                    context.error(self, EvalError::StatementNotSupported(statement.clone()))?;
                }
            }
        }

        context.close_scope();

        if !props.is_complete() {
            use crate::diag::PushDiag;
            context.error(self, EvalError::UninitializedProperties(props.get_incomplete_ids()))?;
            return Ok(crate::objects::empty_object());
        } 

        use crate::objects::*;

        // Make a new object node
        let object = object(Object{ name: crate::Id::default(), props });
        for node in nodes {
            object.append(node);
        }
        Ok(object)
    }
}


impl SrcReferrer for ModuleInitDefinition {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl std::fmt::Display for ModuleInitDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "init({parameters}) ", parameters = self.parameters)?;
        write!(f, "{body}", body = self.body)
    }
}

impl PrintSyntax for ModuleInitDefinition {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}ModuleDefinition:", "")?;
        writeln!(f, "{:depth$} Parameters:", "")?;
        self.parameters.print_syntax(f, depth + 2)?;
        writeln!(f, "{:depth$} Body:", "")?;
        self.body.print_syntax(f, depth + 2)
    }
}
