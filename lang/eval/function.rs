// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Function call evaluation

use crate::{eval::*, syntax::*};

impl CallTrait for FunctionDefinition {
    fn call(
        &self,
        args: &super::ArgumentValueList,
        context: &mut super::Context,
    ) -> super::EvalResult<crate::value::Value> {
        eval_todo!(context, args, "Function evaluation")
    }
}
