use std::io::Read;

use crate::{errors::*, eval::*, parse::*, parser::*};
use microcad_render::tree;

/// µCAD source file
#[derive(Clone, Debug)]
pub struct SourceFile {
    /// Root code body
    pub body: Vec<ModuleStatement>,
    /// Name of loaded file or `None`
    pub filename: Option<std::path::PathBuf>,
    /// Source file string, TODO: might be a &'a str in the future
    _source: String,
}

impl SourceFile {
    /// Load µCAD source file from given `path`
    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let mut file = std::fs::File::open(&path)?;
        let mut buf = String::new();
        use anyhow::Context;
        file.read_to_string(&mut buf)
            .context("Cannot load source file")?;
        let mut source_file =
            Parser::parse_rule::<SourceFile>(crate::parser::Rule::source_file, &buf)
                .context("Could not parse file")?;
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
                Rule::module_statement => {
                    body.push(ModuleStatement::parse(pair)?);
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

    let source_file = SourceFile::from_file(r#"../tests/std/algorithm_difference.µcad"#);
    if let Err(ref err) = source_file {
        eprintln!("{err}");
    }

    let source_file = source_file.unwrap();

    let first_statement = source_file.body.first().unwrap();
    match first_statement {
        ModuleStatement::Use(u) => {
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
    let source_file = SourceFile::from_file("I do not exist.µcad");
    if let Err(err) = source_file {
        eprintln!("{err}");
        //assert_eq!(format!("{err}"), "Cannot load source file");
    } else {
        panic!("Does file exist?");
    }
}
