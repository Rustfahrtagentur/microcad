type Child = std::rc::Rc<std::cell::RefCell<Tree>>;
type Children = std::collections::HashMap<String, Child>;

/// tree catching markdown tests into a valid rust module structure
#[derive(Debug)]
pub enum Tree {
    Root(Children),
    Module(String, Children),
    Test(String, String),
}

impl Tree {
    /// create empty tree
    pub fn new() -> Self {
        Self::Root(Children::new())
    }

    /// insert new test code by module path
    /// - `path`: list of nested rust module names separated by `.`
    /// - `code`: µCAD test code
    pub fn insert(&mut self, path: &str, code: String) {
        use std::{cell::RefCell, rc::Rc};

        if let Some((module, path)) = path.split_once(".") {
            match self {
                Tree::Root(ref mut children) | Tree::Module(_, ref mut children) => {
                    if let Some(ref mut module) = children.get(module) {
                        module.borrow_mut().insert(path, code);
                    } else {
                        _ = children.insert(module.into(), {
                            let mut new = Self::Module(module.into(), Children::new());
                            // recursively fill module
                            new.insert(path, code);
                            Rc::new(RefCell::new(new))
                        })
                    }
                }
                _ => unreachable!(),
            }
        } else {
            match self {
                Tree::Module(_, ref mut children) | Tree::Root(ref mut children) => {
                    _ = children.insert(
                        path.into(),
                        Rc::new(RefCell::new(Tree::Test(path.to_string(), code))),
                    )
                }
                _ => unreachable!(),
            }
        }
    }
}

impl std::fmt::Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tree::Test(name, code) => writeln!(
                f,
                "{}",
                format!(
                    r##"#[test]
#[allow(non_snake_case)]
fn r#{name}() {{
    use crate::{{language::document::Document,parser}};
    let _ = parser::Parser::parse_rule_or_panic::<Document>(
        parser::Rule::document,
        r#"
{code}"#
    );
}}"##
                )
                .trim()
            )?,
            Tree::Root(children) => {
                for child in children {
                    writeln!(f, "{}", child.1.as_ref().borrow().to_string().trim())?;
                }
            }
            Tree::Module(name, children) => {
                writeln!(f, "#[allow(non_snake_case)]\n")?;
                writeln!(f, r##"mod r#{name} {{"##)?;
                for child in children {
                    writeln!(f, "{}", child.1.as_ref().borrow().to_string().trim())?;
                }
                writeln!(f, "}}\n")?;
            }
        }
        Ok(())
    }
}
