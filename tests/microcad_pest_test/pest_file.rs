// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Pest rule

use crate::{PestResult, PestTest, RustWriter};

/// Single pest rule with several tests
pub struct PestRule {
    /// Name of the rule
    pub name: String,
    /// List of tests
    pub tests: Vec<PestTest>,
    /// line number in Pest file
    pub line: Option<usize>,
}

/// Pest grammar file
pub struct PestFile(Vec<PestRule>);

impl std::ops::Deref for PestFile {
    type Target = Vec<PestRule>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PestFile {
    /// Read  content of file int a string
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        use std::{fs::*, io::*};

        // Read file line by line
        let mut buf = String::new();
        // Read file to string
        File::open(path)
            .expect("cannot open PEST grammar file {path}")
            .read_to_string(&mut buf)
            .expect("cannot read PEST grammar from file {path}");

        buf.parse()
    }

    /// generate rust test
    pub fn generate_test_rs(
        &self,
        parser_struct_name: &str,
        rule_enum_name: &str,
        w: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        let mut r = RustWriter::new(w);
        r.writeln("mod grammar {")?;
        r.writeln("use log::error;")?;

        // Generate tests for each rule
        for rule in self.iter() {
            // Do not generate tests for rules without tests
            if rule.tests.is_empty() {
                continue;
            }

            r.write("#[test]")?;
            r.begin_scope(&format!("fn rule_{}()", rule.name))?;
            r.writeln(&format!("let _rule_line = {};", rule.line.unwrap()))?;

            for test in &rule.tests {
                r.begin_scope("")?;
                r.writeln("use pest::Parser;")?;
                r.writeln(&format!("let test_line = {};", test.line.unwrap()))?;
                r.writeln(&format!("let input = r#\"{}\"#;", test.source))?;
                r.begin_scope(
                    std::format!(
                        "#[allow(clippy::single_match)]
                        match {parser_struct_name}::parse({rule_enum_name}::r#{}, input)",
                        rule.name
                    )
                    .as_str(),
                )?;

                match &test.result {
                    PestResult::Ok(s) => {
                        r.writeln("Ok(pairs) =>  assert_eq!(input, pairs.as_str()),")?;
                        r.writeln(&format!(
                            "Err(e) => panic!(\"{{}} at `{{}}`:{{}} {}\", e, input, test_line),",
                            s
                        ))?;
                    }
                    PestResult::Err(s) => {
                        r.begin_scope("Ok(pairs) =>")?;
                        r.begin_scope("if input == pairs.as_str()")?;
                        r.writeln(&format!("panic!(\"Expected parsing error at `{{}}`:{{}}: {}\", input, test_line);", s))?;
                        r.end_scope()?;

                        r.end_scope()?;
                        r.writeln("Err(_) => (),")?;
                    }
                }
                r.end_scope()?;
                r.end_scope()?;
            }
            r.end_scope()?;
        }
        r.writeln("}")?;
        Ok(())
    }
}

impl std::str::FromStr for PestTest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.trim_start().starts_with("//`") {
            return Err(());
        }
        // Read until next `
        let mut iter = s
            .chars()
            .position(|c| c == '`')
            .map(|i| s.chars().skip(i + 1))
            .unwrap();
        let mut test_str = String::new();
        for c in iter.by_ref() {
            if c == '`' {
                break;
            }
            test_str.push(c);
        }

        // Read until :
        for c in iter.by_ref() {
            if c == ':' {
                break;
            }
        }

        // Read until end of line
        let mut result_str = String::new();
        for c in iter {
            if c == '\n' {
                break;
            }
            result_str.push(c);
        }
        let result = result_str.parse::<PestResult>()?;

        Ok(Self {
            source: test_str,
            result,
            line: None,
        })
    }
}

impl std::str::FromStr for PestFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();
        let mut tests = Vec::new();

        for (line_count, line) in s.lines().enumerate() {
            let line_count = line_count + 1;
            // If line is a test comment, add to tests
            if let Ok(mut test) = line.parse::<PestTest>() {
                test.line = Some(line_count);
                tests.push(test);
            } else if line.starts_with("//") {
                // Skip other comments
            } else if let Some(tokens) = line.split_once('=') {
                // read parser rule
                let name = tokens.0.trim();
                // Check if name is identifier
                if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
                    return Err(format!("Invalid rule name: {name}"));
                }
                rules.push(PestRule {
                    name: name.to_string(),
                    tests: tests.clone(),
                    line: Some(line_count),
                });
                tests.clear();
            }
        }

        Ok(Self(rules))
    }
}
