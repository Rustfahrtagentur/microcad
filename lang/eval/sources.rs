// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Evaluation all sources.

use crate::{eval::*, resolve::*};

impl Eval for Sources {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        self.source_files
            .iter()
            // skip root
            .filter(|source| source.hash != self.root().hash)
            .map(|source_file| source_file.eval(context))
            .collect()
    }
}
