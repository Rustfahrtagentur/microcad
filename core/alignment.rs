// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::str::FromStr;

use crate::CoreError;

/// Alignment
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Alignment {
    /// Do not align.
    Fix,
    /// Left, top & front.
    Near,
    /// Center.
    Center,
    /// Right, bottom & back.
    Far,
}

impl std::fmt::Display for Alignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Alignment::Fix => write!(f, "FIX"),
            Alignment::Near => write!(f, "NEAR"),
            Alignment::Center => write!(f, "CENTER"),
            Alignment::Far => write!(f, "FAR"),
        }
    }
}

impl FromStr for Alignment {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FIX" => Ok(Alignment::Fix),
            "NEAR" => Ok(Alignment::Near),
            "CENTER" => Ok(Alignment::Center),
            "FAR" => Ok(Alignment::Far),
            _ => Err(CoreError::BadAlignment(s.to_string())),
        }
    }
}
