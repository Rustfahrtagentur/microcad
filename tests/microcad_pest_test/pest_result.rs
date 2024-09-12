// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pest test result

/// Test result
#[derive(Debug, Clone, PartialEq)]
pub enum PestResult {
    /// Ok
    Ok(String),
    /// Error
    Err(String),
}

impl std::str::FromStr for PestResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <PestResult as std::str::FromStr>::Err> {
        let tokens = s.splitn(2, '#').collect::<Vec<_>>();
        if tokens.is_empty() {
            return Err(());
        }

        let result = tokens.first().unwrap().trim();
        let comment = tokens.get(1).unwrap_or(&"").trim().to_string();

        match result {
            "ok" => Ok(Self::Ok(comment)),
            "error" => Ok(Self::Err(comment)),
            _ => Err(()),
        }
    }
}
