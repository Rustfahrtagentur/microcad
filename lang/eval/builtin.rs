// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use std::rc::Rc;

use custom_debug::Debug;
use microcad_core::{Geometry2D, Geometry3D};
use strum::Display;

use crate::{
    eval::*,
    model::{
        render::{RenderCache, RenderResult},
        *,
    },
    syntax::*,
};

/// Builtin function type
pub type BuiltinFn =
    dyn Fn(Option<&ParameterValueList>, &ArgumentValueList, &mut Context) -> EvalResult<Value>;

/// Builtin function struct
#[derive(Clone)]
pub struct Builtin {
    /// Name of the builtin function
    pub id: Identifier,

    /// Optional parameter value list to check the builtin signature.
    pub parameters: Option<ParameterValueList>,

    /// Functor to evaluate this function
    pub f: &'static BuiltinFn,
}

impl std::fmt::Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "__builtin::{}", &self.id)
    }
}

impl Builtin {
    /// Return identifier
    pub fn id(&self) -> Identifier {
        self.id.clone()
    }
}

impl CallTrait for Builtin {
    /// Call builtin function with given parameter
    /// # Arguments
    /// - `args`: Function arguments
    /// - `context`: Execution context
    fn call(&self, args: &ArgumentValueList, context: &mut Context) -> EvalResult<Value> {
        (self.f)(self.parameters.as_ref(), args, context)
    }
}

/// The kind of the built-in workbench determines its output.
#[derive(Debug, Clone, Display)]
pub enum BuiltinWorkbenchKind {
    /// A parametric 2D primitive.
    Primitive2D,
    /// A parametric 3D primitive.
    Primitive3D,
    /// An affine transformation.
    Transform,
    /// An operation on a model.
    Operation,
}

/// The output of a
pub enum BuiltinWorkpieceOutput {
    Geometry2D(Geometry2D),
    Geometry3D(Geometry3D),
    Transform(AffineTransform),
    Operation(Box<dyn Operation>),
}

/// Builtin sketch function type
pub type BuiltinWorkpieceFn = dyn Fn(&Tuple) -> RenderResult<BuiltinWorkpieceOutput>;

#[derive(Clone, Debug)]
pub struct BuiltinWorkpiece {
    pub kind: BuiltinWorkbenchKind,
    pub creator: Symbol,
    pub args: Tuple,
    #[debug(skip)]
    pub f: &'static BuiltinWorkpieceFn,
}

impl BuiltinWorkpiece {
    pub fn call_2d(&self, cache: &mut RenderCache, model: &Model) -> RenderResult<Rc<Geometry2D>> {
        Ok(match (self.f)(&self.args)? {
            BuiltinWorkpieceOutput::Geometry2D(geo2d) => Rc::new(geo2d),
            BuiltinWorkpieceOutput::Geometry3D(_) => todo!(),
            BuiltinWorkpieceOutput::Transform(_) => todo!(),
            BuiltinWorkpieceOutput::Operation(operation) => operation.process_2d(cache, model)?,
        })
    }
}

impl std::fmt::Display for BuiltinWorkpiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{kind} {symbol}{args}",
            kind = self.kind,
            symbol = self.creator,
            args = self.args
        )
    }
}

/// Builtin part definition
pub trait BuiltinWorkbenchDefinition {
    /// Get id of the builtin part
    fn id() -> &'static str;

    /// The kind of the built-in workbench.
    fn kind() -> BuiltinWorkbenchKind;

    /// The function that generates an output from the workpiece.
    fn workpiece_function() -> &'static BuiltinWorkpieceFn;

    fn workpiece(args: &Tuple) -> BuiltinWorkpiece {
        BuiltinWorkpiece {
            kind: Self::kind(),
            creator: Self::symbol(),
            args: args.clone(),
            f: Self::workpiece_function(),
        }
    }

    /// Create model from argument map
    fn model(args: &Tuple) -> Model {
        ModelBuilder::new(
            Element::BuiltinWorkpiece(Self::workpiece(args)),
            SrcRef(None),
        )
        .build()
    }

    /// Workbench function
    fn function() -> &'static BuiltinFn {
        &|params, args, _| {
            log::trace!(
                "Built-in workbench {call} {id:?}({args})",
                call = crate::mark!(CALL),
                id = Self::id()
            );
            Ok(Value::Models(
                ArgumentMatch::find_multi_match(
                    args,
                    params.expect("A built-in part must have a parameter list"),
                )?
                .iter()
                .map(Self::model)
                .collect(),
            ))
        }
    }

    /// Part initialization parameters
    fn parameters() -> ParameterValueList {
        ParameterValueList::default()
    }

    /// Create builtin symbol
    fn symbol() -> Symbol {
        Symbol::new_builtin(
            Identifier::no_ref(Self::id()),
            Some(Self::parameters()),
            Self::function(),
        )
    }
}
