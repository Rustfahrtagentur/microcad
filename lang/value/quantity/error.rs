// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use thiserror::Error;

use crate::value::Quantity;

#[derive(Debug, Error)]
pub enum QuantityError {}

pub type QuantityResult = Result<Quantity, QuantityError>;
