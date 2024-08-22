use anyhow::Result;
use std::{cell::RefCell, fs, rc::Rc};
use walk_path::*;

pub fn scan_folder(path: impl AsRef<std::path::Path>) -> Result<()> {
    let mut wp = WalkPath::new();
    wp.scan(path.as_ref(), "rs", &scan_file)?;
    Ok(())
}

pub fn scan_file(wp: &mut WalkPath<syn::File>, path: &std::path::Path) -> Result<bool> {
    eprintln!("******** {path:?}");
    let content = fs::read_to_string(path)?;
    match wp {
        WalkPath::Dir(_, ref mut children) | WalkPath::Root(ref mut children) => {
            _ = children.insert(
                path.into(),
                Rc::new(RefCell::new(WalkPath::File(
                    path.into(),
                    syn::parse_file(&content)?,
                ))),
            )
        }
        _ => unreachable!(),
    }
    Ok(false)
}
