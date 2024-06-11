use std::io::Read;

#[derive(Debug, Clone, PartialEq)]
enum PestResult {
    Ok,
    Err,
}

impl std::str::FromStr for PestResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <PestResult as std::str::FromStr>::Err> {
        match s.trim() {
            "ok" => Ok(Self::Ok),
            "err" => Ok(Self::Err),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct PestTest {
    source: String,
    result: PestResult,
}

struct PestRule {
    name: String,
    tests: Vec<PestTest>,
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
        })
    }
}

impl std::str::FromStr for PestFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = Vec::new();
        let mut tests = Vec::new();

        for line in s.lines() {
            if let Ok(test) = line.parse::<PestTest>() {
                println!("{:?}", test);
                tests.push(test);
                continue;
            }

            if let Some(tokens) = line.split_once('=') {
                let name = tokens.0.trim();
                // Check if name is identifier
                if name.chars().any(|c| !c.is_alphanumeric() && c != '_') {
                    return Err(format!("Invalid rule name: {}", name));
                }
                rules.push(PestRule {
                    name: name.to_string(),
                    tests: tests.clone(),
                });
                tests.clear();
            }
        }

        Ok(Self { rules })
    }
}

impl PestFile {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        // Read file line by line
        let mut buf = String::new();
        // Read file to string
        let mut file = std::fs::File::open(path).unwrap();
        file.read_to_string(&mut buf).unwrap();

        buf.parse()
    }

    pub fn generate_test_rs(
        &self,
        parser_struct_name: &str,
        w: &mut impl std::io::Write,
    ) -> Result<(), std::io::Error> {
        writeln!(w, "#[cfg(test)]")?;
        writeln!(w, "mod tests {{")?;
        writeln!(w, "    use super::*;")?;
        for rule in &self.rules {
            writeln!(w, "    #[test]")?;
            writeln!(w, "    fn test_{}() {{", rule.name)?;
            for test in &rule.tests {
                writeln!(w, "        {{")?;
                writeln!(w, "            let input = r#\"{}\"#;", test.source)?;
                writeln!(
                    w,
                    "            match {}::parse(Rule::{}, input) {{",
                    parser_struct_name, rule.name
                )?;

                match test.result {
                    PestResult::Ok => {
                        writeln!(w, "                Ok(_) => (),")?;
                        writeln!(w, "                Err(e) => panic!(\"{{}}\", e),")?;
                    }
                    PestResult::Err => {
                        writeln!(w, "                Ok(_) => panic!(\"Expected error\"),")?;
                        writeln!(w, "                Err(_) => (),")?;
                    }
                }
                writeln!(w, "            }}")?;
                writeln!(w, "        }}")?;
            }
            writeln!(w, "    }}")?;
        }
        writeln!(w, "}}")?;
        Ok(())
    }
}

pub fn generate(parser_struct_name: &str, grammar_file: impl AsRef<std::path::Path>) {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("test.rs");

    PestFile::from_file(&grammar_file)
        .unwrap()
        .generate_test_rs(
            parser_struct_name,
            &mut std::fs::File::create(dest_path).unwrap(),
        )
        .unwrap();
    println!("cargo:rerun-if-changed={}", grammar_file.as_ref().display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comment() {
        let test = r#"//`test`: ok"#;
        let test = test.parse::<PestTest>().unwrap();
        assert_eq!(test.source, "test");
        assert_eq!(test.result, PestResult::Ok);
    }

    #[test]
    fn parse_pest_file() {
        let test = r#"
            //`test1`: ok
            //`test2`: err
            expr = {  "{" ~ expr_interior ~ "}" }
        "#;

        let test = test.parse::<PestFile>().unwrap();
        assert_eq!(test.rules.len(), 1);
        assert_eq!(test.rules[0].name, "expr");
        assert_eq!(test.rules[0].tests.len(), 2);
        assert_eq!(test.rules[0].tests[0].source, "test1");
        assert_eq!(test.rules[0].tests[0].result, PestResult::Ok);
        assert_eq!(test.rules[0].tests[1].source, "test2");
        assert_eq!(test.rules[0].tests[1].result, PestResult::Err);
    }
}
