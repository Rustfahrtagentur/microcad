#[derive(Debug, Clone, PartialEq)]
enum PestResult {
    Ok(String),
    Err(String),
}

impl std::str::FromStr for PestResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <PestResult as std::str::FromStr>::Err> {
        let tokens = s.splitn(2, '#').collect::<Vec<_>>();
        if tokens.is_empty() {
            return Err(());
        }

        let result = tokens.first().unwrap().trim();
        let comment = tokens.get(1).unwrap_or(&"").trim().to_string();

        match result {
            "ok" => Ok(Self::Ok(comment)),
            "error" => Ok(Self::Err(comment)),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct PestTest {
    source: String,
    result: PestResult,
    line: Option<usize>,
}

struct PestRule {
    name: String,
    tests: Vec<PestTest>,
    line: Option<usize>,
}

pub struct PestFile {
    rules: Vec<PestRule>,
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

        Ok(Self { rules })
    }
}
struct RustWriter<'a> {
    w: &'a mut dyn std::io::Write,
    indent: usize,
}

impl<'a> RustWriter<'a> {
    pub fn new(w: &'a mut dyn std::io::Write) -> Self {
        Self { w, indent: 0 }
    }

    pub fn begin_scope(&mut self, s: &str) -> Result<(), std::io::Error> {
        if s.is_empty() {
            self.writeln("{")?;
        } else {
            self.writeln(format!("{s} {{").as_str())?;
        }

        self.indent += 1;
        Ok(())
    }

    pub fn write(&mut self, s: &str) -> Result<(), std::io::Error> {
        self.writeln(s)?;
        Ok(())
    }

    pub fn end_scope(&mut self) -> Result<(), std::io::Error> {
        self.writeln("}")?;
        self.indent -= 1;
        Ok(())
    }

    pub fn writeln(&mut self, s: &str) -> Result<(), std::io::Error> {
        write!(self.w, "{}", "    ".repeat(self.indent))?;
        writeln!(self.w, "{}", s)?;
        Ok(())
    }
}

impl PestFile {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        use std::{fs::*, io::*};

        // Read file line by line
        let mut buf = String::new();
        // Read file to string
        File::open(path).unwrap().read_to_string(&mut buf).unwrap();

        buf.parse()
    }

    pub fn generate_test_rs(
        &self,
        parser_struct_name: &str,
        rule_enum_name: &str,
        w: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        let mut r = RustWriter::new(w);
        r.writeln("mod grammar {")?;
        r.writeln("use crate::*;")?;

        // Generate tests for each rule
        for rule in &self.rules {
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
                        "match {parser_struct_name}::parse({rule_enum_name}::r#{}, input)",
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

pub fn generate(
    parser_struct_name: &str,
    rule_enum_name: &str,
    grammar_file: impl AsRef<std::path::Path>,
) {
    use std::{env::*, fs::*, path::*};

    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("microcad_pest_test.rs");

    PestFile::from_file(&grammar_file)
        .unwrap()
        .generate_test_rs(
            parser_struct_name,
            rule_enum_name,
            &mut File::create(dest_path).unwrap(),
        )
        .unwrap();
    println!("cargo:rerun-if-changed={}", grammar_file.as_ref().display());
}

//pub fn generate_test_case_from_file(test_file: impl AsRef<std::path::Path>)

#[test]
fn test_comment() {
    let test = r#"//`test`: ok # Test"#;
    let test = test.parse::<PestTest>().unwrap();
    assert_eq!(test.source, "test");
    assert_eq!(test.result, PestResult::Ok("Test".into()));
}

#[test]
fn parse_pest_file() {
    let test = r#"
            //`test1`: ok # Ok Test
            //`test2`: error # Error Test
            expr = {  "{" ~ expr_interior ~ "}" }
        "#;

    let test = test.parse::<PestFile>().unwrap();
    assert_eq!(test.rules.len(), 1);
    assert_eq!(test.rules[0].name, "expr");
    assert_eq!(test.rules[0].tests.len(), 2);
    assert_eq!(test.rules[0].tests[0].source, "test1");
    assert_eq!(
        test.rules[0].tests[0].result,
        PestResult::Ok("Ok Test".into())
    );
    assert_eq!(test.rules[0].tests[1].source, "test2");
    assert_eq!(
        test.rules[0].tests[1].result,
        PestResult::Err("Error Test".into())
    );
}
