// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(missing_docs)]

use std::path::PathBuf;

pub struct Output {
    pub name: String,
    pub input_path: PathBuf,
    pub line_no: usize,
    pub test_path: PathBuf,
}

impl Output {
    pub fn has_path(&self, path: &PathBuf) -> bool {
        self.banner_path() == *path || self.out_path() == *path || self.log_path() == *path
    }

    pub fn input_path_str(&self) -> String {
        self.input_path
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    pub fn banner_name(&self) -> String {
        format!("{name}.png", name = self.name)
    }
    pub fn banner_path(&self) -> PathBuf {
        self.test_path.join(self.banner_name())
    }
    pub fn banner_path_str(&self) -> String {
        self.banner_path()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    pub fn out_name(&self) -> String {
        format!("{name}.svg", name = self.name)
    }
    pub fn out_path(&self) -> PathBuf {
        self.test_path.join(self.out_name())
    }
    pub fn out_path_str(&self) -> String {
        self.out_path()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    pub fn log_name(&self) -> String {
        format!("{name}.md", name = self.name)
    }
    pub fn log_path(&self) -> PathBuf {
        self.test_path.join(self.log_name())
    }
    pub fn log_path_str(&self) -> String {
        self.log_path()
            .to_string_lossy()
            .escape_default()
            .to_string()
    }

    pub fn banner(&self) -> String {
        format!(
            "| [![test]({banner})]({log}) | [{name}]({path}) |",
            name = self.name,
            banner = self.banner_path_str(),
            // TODO: out = self.out.as_os_str().to_str().expect(M),
            path = self.input_path_str(),
            log = self.log_path_str()
        )
    }

    pub fn reference(&self) -> String {
        format!("{}#L{}", self.input_path.to_string_lossy(), self.line_no)
    }
}

impl Eq for Output {}

impl PartialEq for Output {
    fn eq(&self, other: &Self) -> bool {
        self.name.to_lowercase().eq(&other.name.to_lowercase())
    }
}

#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for Output {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name
            .to_lowercase()
            .partial_cmp(&other.name.to_lowercase())
    }
}

impl Ord for Output {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.to_lowercase().cmp(&other.name.to_lowercase())
    }
}
