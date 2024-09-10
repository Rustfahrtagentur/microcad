// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µCAD source file representation

use std::io::Read;

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};
use microcad_render::tree;


/// µCAD source file statement
#[derive(Clone, Debug)]
pub enum SourceFileStatement {
    /// Use statement, e.g. `use * from std;`
    Use(UseStatement),
    /// Module definition, e.g. `module foo(r: scalar) { info("Hello, world, {r}!"); }`
    ModuleDefinition(std::rc::Rc<ModuleDefinition>),
    /// Namespace definition, e.g. `namespace foo { }`
    NamespaceDefinition(std::rc::Rc<NamespaceDefinition>),
    /// Function definition, e.g. `fn foo() { }`
    FunctionDefinition(std::rc::Rc<FunctionDefinition>),
    /// Assignment statement, e.g. `a = 10;`
    Assignment(Assignment),
    /// For loop, e.g. `for i in 0..10 { }`
    For(ForStatement),
    /// Expression statement, e.g. `a + b;`
    Expression(Expression),
}

impl SrcReferrer for SourceFileStatement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Use(u) => u.src_ref(),
            Self::ModuleDefinition(m) => m.src_ref(),
            Self::NamespaceDefinition(n) => n.src_ref(),
            Self::FunctionDefinition(f) => f.src_ref(),
            Self::Assignment(a) => a.src_ref(),
            Self::For(f) => f.src_ref(),
            Self::Expression(e) => e.src_ref(),
        }
    }
}

impl Parse for SourceFileStatement {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::source_file_statement);
        let first = pair.clone().into_inner().next().unwrap();
        Ok(match first.as_rule() {
            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::module_definition => {
                Self::ModuleDefinition(std::rc::Rc::<ModuleDefinition>::parse(first)?)
            }
            Rule::namespace_definition => {
                Self::NamespaceDefinition(std::rc::Rc::<NamespaceDefinition>::parse(first)?)
            }
            Rule::function_definition => {
                Self::FunctionDefinition(std::rc::Rc::<FunctionDefinition>::parse(first)?)
            }
            Rule::assignment =>
                Self::Assignment(Assignment::parse(first)?),
            Rule::for_statement => Self::For(ForStatement::parse(first)?),
            Rule::expression | Rule::expression_no_semicolon => Self::Expression(Expression::parse(first)?),
            rule => unreachable!(
                "Unexpected source file statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl Eval for SourceFileStatement {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        match self {
            Self::Use(use_statement) => {
                use_statement.eval(context)?;
            }
            Self::Assignment(assignment) => {
                assignment.eval(context)?;
            }
            Self::FunctionDefinition(function_definition) => {
                context.add_function(function_definition.clone());
            }
            Self::ModuleDefinition(module_definition) => {
                context.add_module(module_definition.clone());
            }
            Self::NamespaceDefinition(namespace_definition) => {
                context.add_namespace(namespace_definition.clone());
            }
            Self::For(for_statement) => {
                for_statement.eval(context)?;
            }
            Self::Expression(expression) => {
                expression.eval(context)?;
            }
        }
        Ok(())
    }
}


/// µCAD source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    /// Root code body
    pub body: Vec<SourceFileStatement>,
    /// Name of loaded file or `None`
    pub filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    _source: String,
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

    /// get a specific line
    ///
    /// - `line`: line number beginning at `0`
    pub fn get_line(&self, line: usize) -> Option<&str> {
        self._source.lines().nth(line)
    }
}

impl Parse for SourceFile {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut body = Vec::new();

        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::source_file_statement => {
                    body.push(SourceFileStatement::parse(pair)?);
                }
                Rule::EOI => break,
                _ => {}
            }
        }

        Ok(SourceFile {
            body,
            filename: None,
            _source: pair.as_span().as_str().to_string(),
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
        SourceFileStatement::Use(u) => {
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

