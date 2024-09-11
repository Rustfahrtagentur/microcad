// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD Call related evaluation entities

#[macro_use]
mod call_argument_value;

mod argument_map;
mod call_argument_value_list;

pub use argument_map::*;
pub use call_argument_value::*;
pub use call_argument_value_list::*;
