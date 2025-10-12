// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

mod symbol_definition;
mod symbol_inner;
mod symbol_map;
mod symbols;

pub use symbol_definition::*;
pub(crate) use symbol_map::*;
pub(crate) use symbols::*;

use symbol_inner::*;

use crate::{builtin::*, rc::*, resolve::*, src_ref::*, syntax::*, ty::*, value::*};

/// Symbol
///
/// Every `Symbol` has a [`SymbolDefinition`], a *parent* and *children* stored within a `Rc<RefCell<`[`SymbolInner`]`>`.
/// So `Symbol` is meant as a tree which is used by [`SymbolTable`] to store
/// the resolved symbols by it's original structure in the source code and by it's *id*.
///
/// `Symbol` can be shared as mutable.
#[derive(Clone)]
pub struct Symbol {
    visibility: std::cell::Cell<Visibility>,
    inner: RcMut<SymbolInner>,
}

impl Symbol {
    /// Create new symbol without children.
    /// # Arguments
    /// - `visibility`: Visibility of the symbol
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub fn new(def: SymbolDefinition, parent: Option<Symbol>) -> Self {
        Symbol {
            visibility: std::cell::Cell::new(def.visibility()),
            inner: RcMut::new(SymbolInner {
                def,
                parent,
                ..Default::default()
            }),
        }
    }

    /// Create new symbol without children.
    /// # Arguments
    /// - `visibility`: Visibility of the symbol
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub(super) fn new_with_visibility(
        visibility: Visibility,
        def: SymbolDefinition,
        parent: Option<Symbol>,
    ) -> Self {
        Symbol {
            visibility: std::cell::Cell::new(visibility),
            inner: RcMut::new(SymbolInner {
                def,
                parent,
                ..Default::default()
            }),
        }
    }

    /// Create a symbol node for a built-in.
    /// # Arguments
    /// - `id`: Name of the symbol
    /// - `parameters`: Optional parameter list
    /// - `f`: The builtin function
    pub fn new_builtin(
        id: Identifier,
        parameters: Option<ParameterValueList>,
        f: &'static BuiltinFn,
    ) -> Symbol {
        Symbol::new(
            SymbolDefinition::Builtin(Rc::new(Builtin { id, parameters, f })),
            None,
        )
    }

    /// Print out symbols from that point.
    /// # Arguments
    /// - `f`: Output formatter
    /// - `id`: Overwrite symbol's internal `id` with this one if given (e.g. when using in a map).
    /// - `depth`: Indention depth to use
    pub(super) fn print_symbol(
        &self,
        f: &mut impl std::fmt::Write,
        id: Option<&Identifier>,
        depth: usize,
        debug: bool,
        children: bool,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        let def = &self.inner.borrow().def;
        let full_name = self.full_name();
        let visibility = self.visibility();
        if debug && cfg!(feature = "ansi-color") && self.inner.borrow().used.get().is_none() {
            let checked = if self.inner.borrow().checked {
                " ✓"
            } else {
                ""
            };
            color_print::cwrite!(
                f,
                "{:depth$}<#606060>{visibility:?}{id:?} {def:?} [{full_name:?}]</>{checked}",
                "",
            )?;
        } else {
            write!(f, "{:depth$}{id} {def} [{full_name}]", "",)?;
        }
        if children {
            writeln!(f)?;
            let indent = 4;

            self.with_children(|(id, child)| {
                child.print_symbol(f, Some(id), depth + indent, debug, true)
            })?;
        }
        Ok(())
    }

    /// Insert child and change parent of child to new parent.
    /// # Arguments
    /// - `parent`: New parent symbol (will be changed in child!).
    /// - `child`: Child to insert
    pub fn add_child(parent: &Symbol, child: Symbol) {
        child.inner.borrow_mut().parent = Some(parent.clone());
        let id = child.id();
        parent.inner.borrow_mut().children.insert(id, child);
    }

    /// Insert new child
    ///
    /// The parent of `new_child` wont be changed!
    pub fn insert(&self, id: Identifier, new_child: Symbol) {
        if self
            .inner
            .borrow_mut()
            .children
            .insert(id, new_child)
            .is_some()
        {
            todo!("symbol already existing");
        }
    }

    /// Initially set children.
    ///
    /// Panics if children already exist.
    pub(super) fn set_children(&self, new_children: SymbolMap) {
        assert!(self.inner.borrow().children.is_empty());
        self.inner.borrow_mut().children = new_children;
    }

    /// Get parent symbol.
    pub(super) fn get_parent(&self) -> Option<Symbol> {
        self.inner.borrow().parent.clone()
    }

    /// Set new parent.
    pub(super) fn set_parent(&mut self, parent: Symbol) {
        self.inner.borrow_mut().parent = Some(parent);
    }

    /// Clone this symbol but give the clone another visibility.
    pub(crate) fn clone_with_visibility(&self, visibility: Visibility) -> Self {
        let cloned = self.clone();
        cloned.visibility.set(visibility);
        cloned
    }

    /// Return the internal *id* of this symbol.
    pub(crate) fn id(&self) -> Identifier {
        self.inner.borrow().def.id()
    }

    /// Get any child with the given `id`.
    /// # Arguments
    /// - `id`: Anticipated *id* of the possible child.
    fn get(&self, id: &Identifier) -> Option<Symbol> {
        self.inner.borrow().children.get(id).cloned()
    }

    fn is_deleted(&self) -> bool {
        self.visibility.get() == Visibility::Deleted
    }

    /// True if symbol has any children
    pub(crate) fn is_empty(&self) -> bool {
        self.inner.borrow().children.is_empty()
    }

    /// Return `true` if symbol's visibility is private
    fn visibility(&self) -> Visibility {
        self.visibility.get()
    }

    /// Set symbol's visibility.
    pub(crate) fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility.set(visibility)
    }

    /// Return `true` if symbol's visibility set to is public.
    fn is_public(&self) -> bool {
        matches!(self.visibility(), Visibility::Public)
    }

    /// Search down the symbol tree for a qualified name.
    /// # Arguments
    /// - `name`: Name to search for.
    pub(crate) fn search(&self, name: &QualifiedName) -> ResolveResult<Symbol> {
        log::trace!("Searching {name} in {:?}", self.full_name());

        if let Some(first) = name.first() {
            if let Some(child) = self.get(first) {
                if name.is_single_identifier() && !child.is_deleted() {
                    log::trace!("Found {name:?} in {:?}", self.full_name());
                    Ok(child.clone())
                } else {
                    let name = &name.remove_first();
                    child.search(name)
                }
            } else {
                log::trace!("No child in {:?} while searching for {name:?}", self.id());
                Err(ResolveError::SymbolNotFound(name.clone()))
            }
        } else {
            log::warn!("Cannot search for an anonymous name");
            Err(ResolveError::SymbolNotFound(name.clone()))
        }
    }

    /// check if a private symbol may be declared within this symbol
    pub(super) fn can_const(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::Module(..) | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a value on the stack may be declared within this symbol
    pub(super) fn can_value(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::Function(..)
                | SymbolDefinition::Workbench(..)
                | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a property may be declared within this symbol
    pub(super) fn can_prop(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDefinition::Workbench(..))
    }

    pub(super) fn is_module(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..)
        )
    }

    /// Overwrite any value in this symbol
    pub(crate) fn set_value(&self, new_value: Value) -> ResolveResult<()> {
        match &mut self.inner.borrow_mut().def {
            SymbolDefinition::Constant(.., value) => {
                *value = new_value;
                Ok(())
            }
            _ => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    /// Return file path of top level parent source file.
    pub(super) fn source_path(&self) -> Option<std::path::PathBuf> {
        if let SymbolDefinition::SourceFile(source_file) = &self.inner.borrow().def {
            return source_file
                .filename()
                .parent()
                .map(|path| path.to_path_buf());
        }
        self.inner
            .borrow()
            .parent
            .as_ref()
            .and_then(|parent| parent.source_path())
    }

    /// Mark this symbol as *checked*.
    pub(super) fn set_check(&self) {
        self.inner.borrow_mut().checked = true;
    }

    /// Mark this symbol as *used*.
    pub(crate) fn set_use(&self) {
        let _ = self.inner.borrow().used.set(());
    }

    /// Work with the symbol definition.
    pub(crate) fn with_def<T>(&self, mut f: impl FnMut(&SymbolDefinition) -> T) -> T {
        f(&self.inner.borrow().def)
    }

    /// Work with the mutable symbol definition.
    pub(crate) fn with_def_mut<T>(&self, mut f: impl FnMut(&mut SymbolDefinition) -> T) -> T {
        f(&mut self.inner.borrow_mut().def)
    }

    pub(super) fn is_resolvable(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::SourceFile(..)
                | SymbolDefinition::Module(..)
                | SymbolDefinition::UseAll(..)
                | SymbolDefinition::Alias(..)
        ) && !self.is_deleted()
    }

    pub(super) fn is_link(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::UseAll(..) | SymbolDefinition::Alias(..)
        )
    }

    pub(super) fn is_alias(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDefinition::Alias(..))
    }

    pub(super) fn has_links(&self) -> bool {
        if self.is_link() {
            true
        } else {
            self.inner
                .borrow()
                .children
                .values()
                .any(|symbol| symbol.has_links())
        }
    }

    // Search recursively within symbol **and** in the symbol table (global)
    fn lookup(&self, name: &QualifiedName, context: &ResolveContext) -> ResolveResult<Symbol> {
        match (self.search(name), context.lookup(name)) {
            (Ok(relative), Ok(global)) => {
                if relative == global || relative.is_alias() {
                    Ok(global)
                } else if global.is_alias() {
                    Ok(relative)
                } else {
                    todo!("lookup ambiguous:\n  {relative:?}\n  {global:?}")
                }
            }
            (Ok(symbol), Err(_)) | (Err(_), Ok(symbol)) => Ok(symbol),
            (Err(err), Err(_)) => Err(err),
        }
    }

    /// Create a vector of cloned children.
    fn public_children(&self, visibility: Visibility) -> SymbolMap {
        let inner = self.inner.borrow();

        inner
            .children
            .values()
            .filter(|symbol| {
                if symbol.is_public() {
                    true
                } else {
                    log::trace!("Skipping private symbol:\n{symbol:?}");
                    false
                }
            })
            .map(|symbol| symbol.clone_with_visibility(visibility))
            .map(|symbol| (symbol.id(), symbol))
            .collect()
    }

    /// Resolve aliases and use statements in this symbol.
    pub(super) fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<SymbolMap> {
        log::trace!("resolving: {self}");

        // retrieve symbols from any use statements
        let mut from_self = {
            let inner = self.inner.borrow();

            match &inner.def {
                SymbolDefinition::Alias(visibility, id, name) => {
                    log::trace!("resolving use (as): {self} => {visibility}{id} ({name})");

                    let symbol = if let Some(parent) = &inner.parent {
                        parent.lookup(name, context)?
                    } else {
                        context.lookup(name)?
                    }
                    .clone_with_visibility(*visibility);

                    self.visibility.set(Visibility::Deleted);

                    [(id.clone(), symbol)].into_iter().collect()
                }
                SymbolDefinition::UseAll(visibility, name) => {
                    log::trace!("resolving use all: {self} => {visibility}{name}");

                    self.visibility.set(Visibility::Deleted);

                    let symbols = if let Some(parent) = &inner.parent {
                        parent.lookup(name, context)?
                    } else {
                        context.lookup(name)?
                    }
                    .public_children(*visibility);

                    if !symbols.is_empty() {
                        self.visibility.set(Visibility::Deleted);
                    }
                    symbols
                }
                // skip others
                _ => SymbolMap::default(),
            }
        };

        let resolved = from_self.resolve_all(context)?;
        from_self.extend(resolved.iter().map(|(k, v)| (k.clone(), v.clone())));

        // collect symbols resolved from children
        let from_children = self.inner.borrow().children.resolve_all(context)?;

        self.inner
            .borrow_mut()
            .children
            .extend(from_children.iter().map(|(k, v)| (k.clone(), v.clone())));

        // return symbols collected from self
        Ok(from_self)
    }

    /// check names in symbol definition
    pub(super) fn check(&self, context: &mut ResolveContext) -> ResolveResult<()> {
        if !matches!(self.visibility.get(), Visibility::Deleted) {
            let names = match &self.inner.borrow().def {
                SymbolDefinition::SourceFile(sf) => sf.names(),
                SymbolDefinition::Module(m) => m.names(),
                SymbolDefinition::Workbench(wb) => wb.names(),
                SymbolDefinition::Function(f) => f.names(),
                SymbolDefinition::Alias(..) | SymbolDefinition::UseAll(..) => {
                    log::error!("Resolve Context:\n{context:?}");
                    return Err(ResolveError::ResolveCheckFailed);
                }
                _ => Default::default(),
            };

            let prefix = self.module_name().clone();

            if !names.is_empty() {
                log::debug!("checking symbols: {names}");

                names
                    .iter()
                    .try_for_each(|name| match context.lookup(name) {
                        Ok(_) => Ok::<_, ResolveError>(()),
                        Err(err) => {
                            if context.lookup(&name.with_prefix(&prefix)).is_err() {
                                context.error(name, err)?;
                            }
                            Ok(())
                        }
                    })?;
            }

            // check children
            let children = self.inner.borrow().children.clone();
            children
                .values()
                .try_for_each(|symbol| symbol.check(context))
        } else {
            Ok(())
        }
    }

    fn module_name(&self) -> QualifiedName {
        match self.is_module() {
            true => {
                if let Some(parent) = &self.get_parent() {
                    parent.module_name().with_suffix(&self.id())
                } else {
                    QualifiedName::from_id(self.id())
                }
            }
            false => {
                if let Some(parent) = &self.get_parent() {
                    parent.module_name()
                } else {
                    unreachable!("root must be source file")
                }
            }
        }
    }

    pub(super) fn unchecked(&self, unchecked: &mut Symbols) {
        let inner = self.inner.borrow();
        if !inner.checked {
            unchecked.push(self.clone())
        }
        inner
            .children
            .iter()
            .for_each(|(_, child)| child.unchecked(unchecked));
    }

    pub(super) fn unused(&self, unused: &mut Symbols) {
        let inner = self.inner.borrow();
        if inner.used.get().is_none() {
            unused.push(self.clone())
        }
        inner
            .children
            .iter()
            .for_each(|(_, child)| child.unused(unused));
    }

    pub(crate) fn with_children<E: std::error::Error>(
        &self,
        f: impl FnMut((&Identifier, &Symbol)) -> Result<(), E>,
    ) -> Result<(), E> {
        self.inner.borrow().children.iter().try_for_each(f)
    }

    pub(crate) fn kind(&self) -> String {
        self.inner.borrow().def.kind()
    }

    /// Returns `true` if builtin symbol uses parameter of type Name
    ///
    /// (for assert_valid() and assert_invalid())
    pub(crate) fn uses_raw_name(&self) -> bool {
        self.with_def(|def| match def {
            SymbolDefinition::Builtin(builtin) => {
                if let Some(parameters) = &builtin.parameters {
                    parameters
                        .values()
                        .any(|param| param.type_matches(&Type::Target))
                } else {
                    false
                }
            }
            _ => false,
        })
    }
}

impl FullyQualify for Symbol {
    /// Get fully qualified name.
    fn full_name(&self) -> QualifiedName {
        let id = self.id();
        match &self.get_parent() {
            Some(parent) => {
                let mut name = parent.full_name();
                name.push(id);
                name
            }

            None => {
                let src_ref = id.src_ref();
                QualifiedName::new(vec![id], src_ref)
            }
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Self {
            visibility: std::cell::Cell::new(Visibility::default()),
            inner: RcMut::new(Default::default()),
        }
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        // just compare the pointers - not the content
        self.inner.as_ptr() == other.inner.as_ptr()
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, None, 0, false, false)
    }
}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, None, 0, true, false)
    }
}

#[test]
fn test_symbol_resolve() {
    let root = SourceFile::load_from_str(
        "root",
        "
        use my; 
        x = my::target;

        use my::target; 
        x = target;
        ",
    )
    .expect("parse error");

    let my = SourceFile::load_from_str(
        "my",
        "
        pub const target = 1;
        ",
    )
    .expect("parse error");

    let mut context =
        ResolveContext::test_create(root, ResolveMode::Symbolized).expect("resolve error");
    context.test_add_file(my);
    log::trace!("{:?}", context);
    context.resolve().expect("resolve error");
}
