// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Model

mod model_builder;
mod model_inner;
mod models;
mod origin;

use derive_more::{Deref, DerefMut};
pub use model_builder::*;
pub use model_inner::*;
pub use models::*;
pub use origin::*;

use crate::{GetPropertyValue, diag::WriteToFile, model_tree::*, rc::*, syntax::*, value::*};
use microcad_core::*;
