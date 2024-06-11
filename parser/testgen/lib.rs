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

struct PestFile {
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
    type Err = ();

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
                if name.chars().any(|c| !c.is_alphanumeric()) {
                    return Err(());
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
    fn parse_from_file(path: &impl AsRef<std::path::Path>) -> Self {
        // Read file line by line
        let mut buf = String::new();
        // Read file to string
        let mut file = std::fs::File::open(path).unwrap();
        file.read_to_string(&mut buf).unwrap();

        buf.parse().unwrap()
    }
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
