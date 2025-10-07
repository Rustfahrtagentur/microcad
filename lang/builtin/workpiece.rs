// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin function evaluation entity

use custom_debug::Debug;
use microcad_core::{Geometry2D, Geometry3D};
use strum::Display;

use crate::{
    eval::*, model::*, render::RenderResult, resolve::Symbol, src_ref::SrcRef, syntax::*, value::*,
};

/// Builtin function type
pub type BuiltinFn =
    dyn Fn(Option<&ParameterValueList>, &ArgumentValueList, &mut EvalContext) -> EvalResult<Value>;

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
    fn call(&self, args: &ArgumentValueList, context: &mut EvalContext) -> EvalResult<Value> {
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

/// The return value when calling a built-in workpiece.
pub enum BuiltinWorkpieceOutput {
    /// 2D geometry output.
    Primitive2D(Geometry2D),
    /// 3D geometry output.
    Primitive3D(Geometry3D),
    /// Transformation.
    Transform(AffineTransform),
    /// Operation.
    Operation(Box<dyn Operation>),
}

/// Builtin sketch function type.
pub type BuiltinWorkpieceFn = dyn Fn(&Tuple) -> RenderResult<BuiltinWorkpieceOutput>;

/// The built-in workpiece.
#[derive(Clone, Debug)]
pub struct BuiltinWorkpiece {
    /// Kind of the workpiece.
    pub kind: BuiltinWorkbenchKind,
    /// Output type
    pub output_type: OutputType,
    /// Creator symbol.
    pub creator: Creator,
    /// The function that will be called when the workpiece is rendered.
    #[debug(skip)]
    pub f: &'static BuiltinWorkpieceFn,
}

impl BuiltinWorkpiece {
    /// Call the workpiece with its arguments.
    pub fn call(&self) -> RenderResult<BuiltinWorkpieceOutput> {
        (self.f)(&self.creator.arguments)
    }
}

impl std::fmt::Display for BuiltinWorkpiece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{kind} {creator}",
            kind = self.kind,
            creator = self.creator,
        )
    }
}

/// Builtin part definition
pub trait BuiltinWorkbenchDefinition {
    /// Get id of the builtin part
    fn id() -> &'static str;

    /// The kind of the built-in workbench.
    fn kind() -> BuiltinWorkbenchKind;

    /// The expected output type.
    fn output_type() -> OutputType {
        OutputType::NotDetermined
    }

    /// The function that generates an output from the workpiece.
    fn workpiece_function() -> &'static BuiltinWorkpieceFn;

    /// Construct the workpiece from an argument tuple.
    fn workpiece(creator: Creator) -> BuiltinWorkpiece {
        BuiltinWorkpiece {
            kind: Self::kind(),
            output_type: Self::output_type(),
            creator,
            f: Self::workpiece_function(),
        }
    }

    /// Create model from argument map
    fn model(creator: Creator) -> Model {
        ModelBuilder::new(
            Element::BuiltinWorkpiece(Self::workpiece(creator)),
            SrcRef(None),
        )
        .build()
    }

    /// Workbench function
    fn function() -> &'static BuiltinFn {
        &|params, args, context| {
            log::trace!(
                "Built-in workbench {call} {id:?}({args})",
                call = crate::mark!(CALL),
                id = Self::id()
            );
            Ok(Value::Model(
                ArgumentMatch::find_multi_match(
                    args,
                    params.expect("A built-in part must have a parameter list"),
                )?
                .iter()
                .map(|tuple| Self::model(Creator::new(context.current_symbol(), tuple.clone())))
                .collect::<Models>()
                .to_multiplicity(SrcRef(None)),
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
