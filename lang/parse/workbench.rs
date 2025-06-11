// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, syntax::*};

impl Parse for WorkbenchKind {
    fn parse(pair: Pair) -> ParseResult<Self> {
        match pair.as_str() {
            "part" => Ok(Self::Part),
            "sketch" => Ok(Self::Sketch),
            "op" => Ok(Self::Operation),
            _ => Err(ParseError::UnexpectedToken),
        }
    }
}

impl Parse for Rc<WorkbenchDefinition> {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Ok(WorkbenchDefinition::new(
            pair.find(Rule::attribute_list).unwrap_or_default(),
            pair.find(Rule::workbench_kind).expect("workbench kind"),
            pair.find(Rule::identifier).expect("workbench identifier"),
            pair.find(Rule::parameter_list)
                .expect("workbench parameter_list"),
            pair.find(Rule::body).expect("workbench body"),
            pair.into(),
        ))
    }
}
