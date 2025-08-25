// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*, tree_display::*};
use std::io::Read;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(path: impl AsRef<std::path::Path> + std::fmt::Debug) -> ParseResult<Rc<Self>> {
        let name = QualifiedName::from_id(Identifier::no_ref(
            &path
                .as_ref()
                .file_stem()
                .expect("illegal file name")
                .to_string_lossy(),
        ));
        Self::load_with_name(path, name)
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path> + std::fmt::Debug,
        name: QualifiedName,
    ) -> ParseResult<Rc<Self>> {
        log::trace!("{load} file {path:?} [{name}]", load = crate::mark!(LOAD));

        let mut file = match std::fs::File::open(&path) {
            Ok(file) => file,
            _ => return Err(ParseError::LoadSource(path.as_ref().into())),
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, &buf, 0)?;
        assert_ne!(source_file.hash, 0);
        source_file.filename = Some(path.as_ref().to_path_buf());
        source_file.name = name;
        log::debug!(
            "Successfully loaded file {} to {}",
            path.as_ref().to_string_lossy(),
            source_file.name
        );
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));

        Ok(Rc::new(source_file))
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `crate::from_str!()`.
    pub fn load_from_str(s: &str) -> ParseResult<Rc<Self>> {
        log::trace!("{load} source from string", load = crate::mark!(LOAD));
        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, s, 0)?;
        log::debug!("Successfully loaded source from string");
        source_file.filename = None;
        log::trace!("Syntax tree:\n{}", FormatTree(&source_file));
        Ok(Rc::new(source_file))
    }

    fn calculate_hash(value: &str) -> u64 {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}

impl Parse for SourceFile {
    fn parse(mut pair: Pair) -> ParseResult<Self> {
        // calculate hash over complete file content
        let hash = Self::calculate_hash(pair.as_str());
        pair.set_source_hash(hash);

        Ok(SourceFile {
            statements: crate::find_rule!(pair, statement_list)?,
            filename: Default::default(),
            source: pair.as_span().as_str().to_string(),
            hash,
            name: QualifiedName::default(),
        })
    }
}

#[test]
fn parse_source_file() {
    let source_file = Parser::parse_rule::<SourceFile>(
        Rule::source_file,
        r#"use std::log::info;
            part Foo(r: Scalar) {
                info("Hello, world, {r}!");
            }
            Foo(20.0);
            "#,
        0,
    )
    .expect("test error");

    assert_eq!(source_file.statements.len(), 3);
}
