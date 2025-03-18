// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad Call related evaluation entities

#[macro_use]
mod call_argument_value;
mod call_argument_value_list;
mod call_trait;

pub use call_argument_value::*;
pub use call_argument_value_list::*;
pub use call_trait::*;
