// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{parse::*, parser::*, rc::*};
use std::io::Read;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(path: impl AsRef<std::path::Path>) -> ParseResult<Rc<Self>> {
        Self::load_with_name(path, QualifiedName::default())
    }

    /// Load µcad source file from given `path`
    pub fn load_with_name(
        path: impl AsRef<std::path::Path>,
        name: QualifiedName,
    ) -> ParseResult<Rc<Self>> {
        let mut file = match std::fs::File::open(&path) {
            Ok(file) => file,
            _ => return Err(ParseError::LoadSource(path.as_ref().into())),
        };

        let mut buf = String::new();
        file.read_to_string(&mut buf)?;

        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, &buf, 0)?;
        assert_ne!(source_file.hash, 0);
        source_file.filename = path.as_ref().to_path_buf();
        source_file.name = name;
        log::debug!(
            "Successfully loaded file {}",
            path.as_ref().to_string_lossy(),
        );
        log::trace!("Syntax tree:\n{}", FormatSyntax(&source_file));

        Ok(Rc::new(source_file))
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `"<from_str>"`.
    pub fn load_from_str(s: &str) -> ParseResult<Rc<Self>> {
        use std::{
            hash::{Hash, Hasher},
            str::FromStr,
        };

        // TODO: Would not the hash be calculated in SourceFile::parse anyway?
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        "<from_str>".hash(&mut hasher);
        let hash = hasher.finish();

        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, s, hash)?;
        source_file.filename = std::path::PathBuf::from_str("<from_str>").expect("filename error");
        log::debug!("loaded string successfully",);
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
            statements: match pair
                .inner()
                .find(|pair| pair.as_rule() == Rule::statement_list)
                .map(StatementList::parse)
            {
                Some(Ok(stmts)) => stmts,
                Some(Err(err)) => {
                    return Err(err);
                }
                None => StatementList::default(),
            },
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
        r#"use std::io::println;
            part foo(r: scalar) {
                info("Hello, world, {r}!");
            }
            foo(20.0);
            "#,
        0,
    )
    .expect("test error");

    assert_eq!(source_file.statements.len(), 3);
}
