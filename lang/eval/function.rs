// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Function call evaluation

use crate::{eval::*, syntax::*};

impl CallTrait for FunctionDefinition {
    fn call(
        &self,
        args: &super::CallArgumentValueList,
        context: &mut super::Context,
    ) -> super::EvalResult<crate::value::Value> {
        log::debug!("calling function: {}({args})", self.id);
        self.body.eval(
            args.get_matching_arguments(context, &self.signature.parameters)?
                .into_symbols(),
            context,
        )
    }
}
