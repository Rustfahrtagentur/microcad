use std::{
    ffi::OsStr,
    io::{BufRead, BufWriter, Read, Write},
};

use walkdir::WalkDir;

pub fn generate_test(test_name: &str, listing: &str, w: &mut impl std::io::Write) {
    let template = format!(
        r##"
        #[test]
        fn {test_name}() {{
            let input = r#"{}{listing}"#;
        }}
    "##,
        "\n"
    );

    writeln!(w, "{}", template);
}

pub fn generate_tests_for_md_file(
    md_path: impl AsRef<std::path::Path>,
    w: &mut impl std::io::Write,
) {
    let md_file = std::fs::File::open(md_path).unwrap();

    let buf_reader = std::io::BufReader::new(md_file);

    let mut listing = String::new();
    let mut test_name = None;

    for line in buf_reader.lines().map_while(Result::ok) {
        if line.starts_with("```µcad") {
            listing.clear();
            let tokens = line.split(",").collect::<Vec<_>>();
            if tokens.len() >= 2 {
                test_name = Some(tokens[1].to_string());
            }
        } else if &line == "```" {
            if test_name.is_some() {
                generate_test(test_name.as_ref().unwrap(), &listing, w);
                test_name = None;
            }
        } else if test_name.is_some() {
            listing += &format!("{line}\n");
        }
    }
}

pub fn generate(path: impl AsRef<std::path::Path>) {
    let walker = WalkDir::new(path).into_iter();

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("md_test.rs");

    let rs_file = std::fs::File::create(dest_path).unwrap();
    let mut w = BufWriter::new(rs_file);

    for entry in walker {
        let path = entry.as_ref().unwrap().path();
        if Some(OsStr::new("md")) == path.extension() {
            generate_tests_for_md_file(path, &mut w);
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }

    // regex::Regex::new(r"```µcad(\.\w+)?\n(.*)```").unwrap();
}
