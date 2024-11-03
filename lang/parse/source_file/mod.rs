// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD source file representation

mod statement;

pub use statement::*;

use std::io::Read;

use crate::{errors::*, eval::*, objecttree, parse::*, parser::*, src_ref::*};

/// Trait to get a source file by its hash
pub trait GetSourceFileByHash {
    /// Get a source file by its hash
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile>;

    /// Convenience function to get a source file by from a `SrcRef`
    fn get_source_file_by_src_ref(&self, src_ref: impl SrcReferrer) -> Option<&SourceFile> {
        self.get_source_file_by_hash(src_ref.src_ref().source_hash())
    }

    /// Convenience function to get source slice by `SrcRef`
    fn get_source_string(&self, src_ref: impl SrcReferrer) -> Option<&str> {
        if let Some(source_file) = self.get_source_file_by_src_ref(&src_ref) {
            Some(src_ref.src_ref().source_slice(&source_file.source))
        } else {
            None
        }
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
    source: String,

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

        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, &buf, 0)
            .context("Could not parse file")?;

        assert_ne!(source_file.hash, 0);

        source_file.filename = Some(std::path::PathBuf::from(path.as_ref()));
        Ok(source_file)
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `"<from_str>"`.
    pub fn load_from_str(s: &str) -> anyhow::Result<Self> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        "<from_str>".hash(&mut hasher);
        let hash = hasher.finish();

        Parser::parse_rule(crate::parser::Rule::source_file, s, hash)
    }

    /// Return filename of loaded file or `<no file>`
    pub fn filename_as_str(&self) -> &str {
        self.filename
            .as_ref()
            .map(|path| path.to_str().unwrap_or("<no file>"))
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
        self.source.lines().nth(line)
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
                    namespace.add(Symbol::Value(a.name.id().unwrap(), a.value.eval(context)?));
                }
                Statement::FunctionDefinition(f) => {
                    namespace.add(f.clone().into());
                }
                Statement::ModuleDefinition(m) => {
                    namespace.add(m.clone().into());
                }
                Statement::NamespaceDefinition(n) => {
                    let n = n.eval(context)?;
                    namespace.add(n);
                }
                Statement::Use(u) => {
                    if let Some(symbols) = u.eval(context)? {
                        for (id, symbol) in symbols.iter() {
                            namespace.add_alias(symbol.as_ref().clone(), id.clone());
                        }
                    }
                }

                _ => {}
            }
        }

        Ok(std::rc::Rc::new(namespace))
    }
}

impl Parse for SourceFile {
    fn parse(mut pair: Pair) -> ParseResult<Self> {
        let mut body = Vec::new();

        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        pair.as_str().hash(&mut hasher);
        let hash = hasher.finish();
        pair.set_source_hash(hash);

        for pair in pair.inner() {
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
            source: pair.as_span().as_str().to_string(),
            hash,
        })
    }
}

impl Eval for SourceFile {
    type Output = objecttree::ObjectNode;

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        let mut new_nodes = Vec::new();

        // Descend into root node and find all child nodes
        context.descend_node(crate::objecttree::group(), |context| {
            for statement in &self.body {
                match statement {
                    Statement::Expression(expression) => {
                        // This statement has been evaluated into a new child node.
                        // Add it to our `new_nodes` list
                        if let Value::Node(node) = expression.eval(context)? {
                            new_nodes.push(node);
                        }
                    }
                    statement => {
                        statement.eval(context)?;
                    }
                }
            }

            Ok(context.current_node().clone())
        })?;

        // Append all child nodes to root node
        for node in new_nodes {
            context.append_node(node);
        }

        // Return root node
        Ok(context.current_node().clone())
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
    let source_file = Parser::parse_rule::<SourceFile>(
        Rule::source_file,
        r#"use std::io::println;
            module foo(r: scalar) {
                info("Hello, world, {r}!");
            }
            foo(20.0);
            "#,
        0,
    )
    .unwrap();

    assert_eq!(source_file.body.len(), 3);
}

#[test]
fn load_source_file() {
    use log::*;

    crate::env_logger_init();

    let source_file = SourceFile::load(r#"../tests/test_cases/algorithm_difference.µcad"#);
    if let Err(ref err) = source_file {
        error!("{err}");
    }

    let source_file = source_file.unwrap();

    let first_statement = source_file.body.first().unwrap();
    match first_statement {
        Statement::Use(u) => {
            use crate::src_ref::SrcReferrer;
            assert_eq!(u.src_ref().source_slice(&source_file.source), "use std::*;");
        }
        _ => panic!(),
    }
}

#[test]
fn load_source_file_wrong_location() {
    use log::*;

    crate::env_logger_init();

    let source_file = SourceFile::load("I do not exist.µcad");
    if let Err(err) = source_file {
        info!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
