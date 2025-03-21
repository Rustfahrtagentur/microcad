use crate::{parse::*, parser::*};
use std::io::Read;

impl SourceFile {
    /// Load µcad source file from given `path`
    pub fn load(path: impl AsRef<std::path::Path>) -> ParseResult<std::rc::Rc<Self>> {
        let mut file = match std::fs::File::open(&path) {
            Ok(file) => file,
            _ => return Err(ParseError::LoadSource(path.as_ref().into())),
        };
        let mut buf = String::new();

        file.read_to_string(&mut buf)?;

        let mut source_file: Self = Parser::parse_rule(crate::parser::Rule::source_file, &buf, 0)?;

        assert_ne!(source_file.hash, 0);

        source_file.filename = Some(std::path::PathBuf::from(path.as_ref()));
        Ok(std::rc::Rc::new(source_file))
    }

    /// Create `SourceFile` from string
    /// The hash of the result will be of `"<from_str>"`.
    pub fn load_from_str(s: &str) -> ParseResult<Self> {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        "<from_str>".hash(&mut hasher);
        let hash = hasher.finish();

        Parser::parse_rule(crate::parser::Rule::source_file, s, hash)
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
                Rule::statement => {
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
    .expect("test error");

    assert_eq!(source_file.body.len(), 3);
}
