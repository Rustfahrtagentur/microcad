// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD source file representation

mod statement;

pub use statement::*;

use std::io::Read;

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;

/// Trait to get a source file by its hash
pub trait GetSourceFileByHash {
    /// Get a source file by its hash
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile>;

    /// Convenience function to get a source file by from a `SrcRef`
    fn get_source_file_by_src_ref(&self, src_ref: impl SrcReferrer) -> Option<&SourceFile> {
        self.get_source_file_by_hash(src_ref.src_ref().source_file_hash())
    }
}

/// µCAD source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    /// Root code body
    pub body: Vec<Statement>,
    /// Name of loaded file or `None`
    pub filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    _source: String,

    /// Hash of the source file
    ///
    /// This hash is calculated from the filename or the source code itself
    ///
    /// This is used to map `SrcRef` -> `SourceFile`
    hash: u64,
}

impl SourceFile {
    /// Load µCAD source file from given `path`
    pub fn load(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(&path)?;
        let mut buf = String::new();
        use anyhow::Context;
        file.read_to_string(&mut buf)
            .context("Cannot load source file")?;
        use std::str::FromStr;
        let mut source_file = Self::from_str(&buf).context("Could not parse file")?;

        source_file.filename = Some(std::path::PathBuf::from(path.as_ref()));
        Ok(source_file)
    }

    /// Return filename of loaded file or `<no file>`
    pub fn filename(&self) -> &str {
        self.filename
            .as_ref()
            .map(|p| p.to_str().unwrap_or("<no file>"))
            .unwrap_or("<no file>")
    }

    /// Return source file hash
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self._source.lines().nth(line)
    }

    /// Evaluate the source file as a namespace
    ///
    /// This is used to evaluate the source file as a namespace, which can be used to import
    /// functions and values from the source file.
    /// This functionality is used for the `use` statement.
    ///
    /// TODOs:
    /// - [ ] Test this function
    /// - [ ] Use this function in the `use` statement evaluation
    pub fn eval_as_namespace(
        &self,
        context: &mut Context,
        namespace_name: Identifier,
    ) -> Result<std::rc::Rc<NamespaceDefinition>> {
        let mut namespace = NamespaceDefinition::new(namespace_name);

        for statement in &self.body {
            match statement {
                Statement::Assignment(a) => {
                    namespace.add_value(a.name.id().unwrap(), a.value.eval(context)?);
                }
                Statement::FunctionDefinition(f) => {
                    namespace.add_function(f.clone());
                }
                Statement::ModuleDefinition(m) => {
                    namespace.add_module(m.clone());
                }
                Statement::NamespaceDefinition(n) => {
                    namespace.add_namespace(n.clone());
                }
                _ => {}
            }
        }

        Ok(std::rc::Rc::new(namespace))
    }
}

impl Parse for SourceFile {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut body = Vec::new();

        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        pair.as_str().hash(&mut hasher);
        let hash = hasher.finish();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::source_file_statement => {
                    body.push(Statement::parse(pair)?);
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        Ok(SourceFile {
            body,
            filename: None,
            _source: pair.as_span().as_str().to_string(),
            hash,
        })
    }
}

impl Eval for SourceFile {
    type Output = tree::Node;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let node = tree::root();
        context.set_current_node(node.clone());
        for statement in &self.body {
            statement.eval(context)?;
        }
        Ok(node)
    }
}

/// Implement `FromStr` trait for `SourceFile` to allow parsing from string
impl std::str::FromStr for SourceFile {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        Parser::parse_rule(crate::parser::Rule::source_file, s)
    }
}

/// We can get a source file by its hash
impl GetSourceFileByHash for SourceFile {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        if self.hash == hash {
            Some(self)
        } else {
            None
        }
    }
}

#[test]
fn parse_source_file() {
    let source_file = Parser::parse_rule_or_panic::<SourceFile>(
        Rule::source_file,
        r#"use std::io::println;
            module foo(r: scalar) {
                info("Hello, world, {r}!");
            }
            foo(20.0);
            "#,
    );

    assert_eq!(source_file.body.len(), 3);
}

#[test]
fn load_source_file() {
    eprintln!("{:?}", std::env::current_dir());

    let source_file = SourceFile::load(r#"../tests/std/algorithm_difference.µcad"#);
    if let Err(ref err) = source_file {
        eprintln!("{err}");
    }

    let source_file = source_file.unwrap();

    let first_statement = source_file.body.first().unwrap();
    match first_statement {
        Statement::Use(u) => {
            use crate::src_ref::SrcReferrer;
            assert_eq!(
                u.src_ref().source_slice(&source_file._source),
                "use * from std;"
            );
        }
        _ => panic!(),
    }
}

#[test]
fn load_source_file_wrong_location() {
    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        eprintln!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
