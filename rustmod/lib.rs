use anyhow::Result;
use std::{cell::RefCell, fs, rc::Rc};
use syn::{ItemImpl, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion, ItemUse};
use walk_path::*;

#[derive(Debug)]
pub enum Scan {
    Trait(String),
    TraitAlias(String),
    Impl(String),
    Use(String),
    Type(String),
    Union(String),
    Struct(String),
}

impl std::fmt::Display for Scan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scan::Trait(name) => write!(f, "trait: {name}"),
            Scan::Impl(name) => write!(f, "impl:\n{name}"),
            Scan::Use(name) => write!(f, "use:\n{name}"),
            Scan::Type(name) => write!(f, "type:\n{name}"),
            Scan::Union(name) => write!(f, "union:\n{name}"),
            Scan::Struct(name) => write!(f, "struct:\n{name}"),
            Scan::TraitAlias(name) => write!(f, "trait alias:\n{name}"),
        }
    }
}

pub fn scan_project_files(path: impl AsRef<std::path::Path>) -> Result<WalkPath<Vec<Scan>>> {
    let mut wp = WalkPath::new();
    wp.scan(path.as_ref(), "rs", &scan_file)?;
    Ok(wp)
}

fn scan_file(wp: &mut WalkPath<Vec<Scan>>, path: &std::path::Path) -> Result<bool> {
    let content = fs::read_to_string(path)?;
    match wp {
        WalkPath::Dir(_, ref mut children) | WalkPath::Root(ref mut children) => {
            match syn::parse_file(&content) {
                Ok(parsed) => {
                    let scan = scan(parsed.items);
                    if !scan.is_empty() {
                        _ = children.insert(
                            path.into(),
                            Rc::new(RefCell::new(WalkPath::File(path.into(), scan))),
                        )
                    }
                }
                Err(err) => eprintln!("ERROR: {err:?}"),
            }
        }
        _ => unreachable!(),
    }
    Ok(false)
}

fn scan(items: Vec<syn::Item>) -> Vec<Scan> {
    let mut result = Vec::new();
    for item in &items {
        match item {
            syn::Item::Const(_) => (),
            syn::Item::Enum(_) => (),
            syn::Item::ExternCrate(_) => (),
            syn::Item::Fn(_) => (),
            syn::Item::ForeignMod(_) => (),
            syn::Item::Impl(i) => scan_impl(i, &mut result),
            syn::Item::Macro(_) => (),
            syn::Item::Mod(_) => (),
            syn::Item::Static(_) => (),
            syn::Item::Struct(s) => scan_struct(s, &mut result),
            syn::Item::Trait(t) => scan_trait(t, &mut result),
            syn::Item::TraitAlias(t) => scan_trait_alias(t, &mut result),
            syn::Item::Type(t) => scan_type(t, &mut result),
            syn::Item::Union(u) => scan_union(u, &mut result),
            syn::Item::Use(u) => scan_use(u, &mut result),
            syn::Item::Verbatim(_) => (),
            _ => todo!(),
        }
    }
    result
}

fn scan_impl(i: &ItemImpl, result: &mut Vec<Scan>) {
    if let Some(t) = &i.trait_ {
        let i =
            t.1.segments
                .iter()
                .map(|s| s.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
        result.push(Scan::Impl(i.to_string()));
    }
}

fn scan_trait(t: &ItemTrait, result: &mut Vec<Scan>) {
    result.push(Scan::Trait(t.ident.to_string()));
}

fn scan_trait_alias(t: &ItemTraitAlias, result: &mut Vec<Scan>) {
    result.push(Scan::TraitAlias(t.ident.to_string()));
}

fn scan_use(u: &ItemUse, result: &mut Vec<Scan>) {
    result.push(Scan::Use(format!("{u:?}")));
}

fn scan_type(t: &ItemType, result: &mut Vec<Scan>) {
    result.push(Scan::Type(format!("{t:?}")));
}

fn scan_union(u: &ItemUnion, result: &mut Vec<Scan>) {
    result.push(Scan::Union(format!("{u:?}")));
}

fn scan_struct(s: &ItemStruct, result: &mut Vec<Scan>) {
    result.push(Scan::Struct(format!("{s:?}")));
}

#[test]
fn scan_test() {
    eprintln!("{:#?}", scan_project_files("..").unwrap());
}
