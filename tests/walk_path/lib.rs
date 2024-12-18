// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Walk a path recursively

use anyhow::Result;

/// Tree leaf node
pub type File<T> = std::rc::Rc<std::cell::RefCell<WalkPath<T>>>;
/// Tree node
pub type Dir<T> = std::collections::HashMap<std::path::PathBuf, File<T>>;

/// Walked path
#[derive(Debug)]
pub enum WalkPath<T> {
    /// Root node
    Root(Dir<T>),
    /// Tree node
    Dir(std::path::PathBuf, Dir<T>),
    /// Leaf node
    File(std::path::PathBuf, T),
}

impl<T> Default for WalkPath<T> {
    fn default() -> Self {
        WalkPath::new()
    }
}

impl<T> WalkPath<T> {
    /// create empty tree
    pub fn new() -> Self {
        Self::Root(Dir::new())
    }

    /// recursive directory scanner
    /// returns `false` if no leafs were generated
    ///
    /// # Arguments
    /// - `path`: directory to scan
    /// - `extension`: file extension to scan for
    /// - `f`: function to call for each file found
    ///
    /// # Returns
    /// `true` if any leafs were generated
    pub fn scan(
        &mut self,
        path: &std::path::Path,
        extension: &str,
        exclude_dir: &[&str],
        f: &dyn Fn(&mut WalkPath<T>, &std::path::Path) -> Result<bool>,
    ) -> Result<bool> {
        // prepare return value
        let mut found = false;
        // read given directory
        for entry in std::fs::read_dir(path)?.flatten() {
            // get file type
            if let Ok(file_type) = entry.file_type() {
                let file_name = entry.file_name().into_string().unwrap();
                // check if directory or file
                if file_type.is_dir() && !exclude_dir.contains(&file_name.as_str()) {
                    // scan deeper
                    if self.scan(&entry.path(), extension, exclude_dir, f)? {
                        // found something
                        found = true;
                    }
                } else if file_type.is_file()
                    && file_name.ends_with(&format!(".{extension}"))
                    && !f(self, &entry.path())?
                {
                    // tell cargo to watch this file
                    println!("cargo:rerun-if-changed={}", entry.path().display());
                    // found something
                    found = true;
                }
            }
        }
        Ok(found)
    }
}
