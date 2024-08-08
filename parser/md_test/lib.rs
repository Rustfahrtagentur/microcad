pub fn generate_test(w: &mut impl std::io::Write, test_name: &str, listing: &str) {
    let template = format!(
        r##"
        #[test]
        fn r#{test_name}() {{
            use crate::*;
            let document = crate::parser::Parser::parse_rule_or_panic::<Document>(
                Rule::document,
                r#"{}{listing}"#
            );

        }}
    "##,
        "\n",
    );
    writeln!(w, "{}", template).unwrap();
}

fn generate_tests_for_md_file(w: &mut impl std::io::Write, md_path: &std::path::Path) {
    use std::{fs::*, io::*};

    let module_name = md_path.file_stem().unwrap().to_str().unwrap().to_string();
    let buf_reader = BufReader::new(File::open(md_path).unwrap());
    let mut listing = String::new();
    let mut test_name = None;

    for line in buf_reader.lines().map_while(Result::ok) {
        if line.to_lowercase().starts_with("```µcad") {
            listing.clear();
            let tokens = line.split(",").collect::<Vec<_>>();
            if tokens.len() >= 2 {
                test_name = Some(tokens[1].to_string());
            }
        } else if &line == "```" {
            if test_name.is_some() {
                generate_test(w, test_name.as_ref().unwrap(), &listing);
                test_name = None;
            }
        } else if test_name.is_some() {
            listing += &format!("{line}\n");
        }
    }
}

pub fn generate(path: impl AsRef<std::path::Path>) {
    use std::{env, fs::*, io::*, path::*};

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("md_test.rs");
    let mut w = BufWriter::new(File::create(dest_path).unwrap());

    fn recurse(w: &mut BufWriter<File>, path: &Path) {
        for entry in read_dir(path).unwrap().flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    writeln!(
                        w,
                        r#"mod r#{module} {{"#,
                        module = entry.file_name().to_string_lossy()
                    )
                    .unwrap();

                    recurse(w, &entry.path());

                    writeln!(w, "}}").unwrap();
                } else if file_type.is_file()
                    && entry.file_name().to_str().unwrap().ends_with(".md")
                {
                    generate_tests_for_md_file(w, &entry.path());
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }

    recurse(&mut w, path.as_ref());
}
