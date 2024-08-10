use tree::Tree;

mod tree;

#[test]
fn test_generate_md_file() {
    use std::path::*;

    let mut tree = Tree::new();
    generate_tests_for_md_file(&mut tree, Path::new("../../doc/namespaces.md"));
    eprintln!("{tree}");
}

pub fn generate(path: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    use std::{env::*, fs::*, io::*, path::*};

    // get target path
    let out_dir = var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("md_test.rs");
    // create target file
    let mut w = BufWriter::new(File::create(dest_path).expect("cannot create file 'md_test.rs'"));

    // read all into a tree to reorder modules
    let mut tree = Tree::new();

    // recursive directory scanner
    fn recurse(w: &mut BufWriter<File>, tree: &mut Tree, path: &Path) -> Result<()> {
        for entry in read_dir(path).unwrap().flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    // begin a rust module
                    writeln!(w, "#[allow(non_snake_case)]\n")?;
                    writeln!(
                        w,
                        r#"mod r#{module} {{"#,
                        module = entry.file_name().to_string_lossy()
                    )?;
                    // scan deeper
                    recurse(w, tree, &entry.path())?;
                    // end the rust module
                    writeln!(w, "}}")?;
                } else if file_type.is_file()
                    && entry.file_name().to_str().unwrap().ends_with(".md")
                {
                    generate_tests_for_md_file(tree, &entry.path());
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
        Ok(())
    }

    recurse(&mut w, &mut tree, path.as_ref())?;

    // generate output rust code
    writeln!(w, "{tree}")
}

fn generate_tests_for_md_file(tree: &mut Tree, path: &std::path::Path) {
    use regex::*;
    use std::{fs::*, io::*};

    // load markdown file
    let mut md_content = String::new();
    {
        File::open(path)
            .expect("file open error")
            .read_to_string(&mut md_content)
            .expect("file read error");
    }

    // match markdown code markers for µcad
    let reg = Regex::new(r#"```µ[Cc][Aa][Dd](,(?<name>[.\w]+))?\n(?<code>[^`]*)+```"#)
        .expect("bad regex");

    let path = path
        .iter()
        .map(|f| f.to_str().unwrap())
        .filter(|f| *f != "..")
        .map(|f| *f.split('.').next().as_ref().unwrap())
        .collect::<Vec<&str>>()
        .join(".");

    for cap in reg.captures_iter(&md_content) {
        // check if code is named
        if let (Some(code), Some(name)) = (cap.name("code"), cap.name("name")) {
            if path.is_empty() {
                tree.insert(name.as_str(), code.as_str().to_string());
            } else {
                tree.insert(
                    &format!("{path}.{}", name.as_str()),
                    code.as_str().to_string(),
                );
            }
        }
    }
}
