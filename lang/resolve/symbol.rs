// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};
use custom_debug::Debug;
use derive_more::{Deref, DerefMut};

/// Symbol content
#[derive(Debug, Clone)]
pub(super) struct SymbolInner {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent
    #[debug(skip)]
    pub parent: Option<Symbol>,
    /// Symbol's children
    pub children: SymbolMap,
    /// Flag if this symbol has been checked after resolving
    pub checked: bool,
    /// Flag if this symbol was in use
    pub used: bool,
}

impl Default for SymbolInner {
    fn default() -> Self {
        Self {
            def: SymbolDefinition::SourceFile(SourceFile::default().into()),
            parent: Default::default(),
            children: Default::default(),
            checked: false,
            used: false,
        }
    }
}

/// Symbol
///
/// Every `Symbol` has a [`SymbolDefinition`], a *parent* and *children* stored within a `Rc<RefCell<`[`SymbolInner`]`>`.
/// So `Symbol` is meant as a tree which is used by [`SymbolTable`] to store
/// the resolved symbols by it's original structure in the source code and by it's *id*.
///
/// `Symbol` can be shared as mutable.
#[derive(Clone)]
pub struct Symbol {
    visibility: Visibility,
    inner: RcMut<SymbolInner>,
}

/// List of qualified names which can pe displayed
#[derive(Debug, Deref, DerefMut, Default)]
pub struct Symbols(Vec<Symbol>);

impl Symbols {
    /// Return all fully qualified names of all symbols.
    #[cfg(test)]
    pub(super) fn full_names(&self) -> QualifiedNames {
        self.iter().map(|symbol| symbol.full_name()).collect()
    }
}

impl FromIterator<Symbols> for Symbols {
    fn from_iter<T: IntoIterator<Item = Symbols>>(iter: T) -> Self {
        let mut symbols = Self::default();
        iter.into_iter()
            .for_each(|mut children| symbols.append(&mut children));
        symbols
    }
}

impl From<Vec<Symbol>> for Symbols {
    fn from(value: Vec<Symbol>) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for Symbols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|symbol| symbol.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

impl FromIterator<Symbol> for Symbols {
    fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Symbol {
    /// Create new symbol without children.
    /// # Arguments
    /// - `visibility`: Visibility of the symbol
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub fn new(def: SymbolDefinition, parent: Option<Symbol>) -> Self {
        Symbol {
            visibility: def.visibility(),
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
            visibility,
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
    pub fn print_symbol(
        &self,
        f: &mut impl std::fmt::Write,
        id: Option<&Identifier>,
        depth: usize,
        debug: bool,
        children: bool,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        if debug && cfg!(feature = "ansi-color") && !self.inner.borrow().used {
            color_print::cwrite!(
                f,
                "{:depth$}<#606060>{visibility}{id:?} {def} [{full_name:?}]</>{checked}",
                "",
                visibility = self.visibility(),
                def = self.inner.borrow().def,
                full_name = self.full_name(),
                checked = if self.inner.borrow().checked {
                    " ✓"
                } else {
                    ""
                }
            )?;
        } else {
            write!(
                f,
                "{:depth$}{id} {} [{}]",
                "",
                self.inner.borrow().def,
                self.full_name(),
            )?;
        }
        if children {
            writeln!(f)?;
            let indent = 4;

            self.inner
                .borrow()
                .children
                .iter()
                .try_for_each(|(id, child)| {
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
    pub fn set_children(&self, new_children: SymbolMap) {
        assert!(self.inner.borrow().children.is_empty());
        self.inner.borrow_mut().children = new_children;
    }

    /// Set new parent.
    pub fn set_parent(&mut self, parent: Symbol) {
        self.inner.borrow_mut().parent = Some(parent);
    }

    /// Move all children from another symbol into this one.
    /// # Arguments
    /// - `from`: Append this symbol's children
    ///
    /// Technically, nothing will be moved here because of the `Rc<RefCell<>>`,
    /// but by resetting the parent of all moved  children, those will see
    /// themselves as child of `self` (e.g when providing fully qualified name).
    pub fn move_children(&self, from: &Symbol) {
        // copy children
        from.inner.borrow().children.iter().for_each(|(id, child)| {
            child.inner.borrow_mut().parent = Some(self.clone());
            self.inner
                .borrow_mut()
                .children
                .insert(id.clone(), child.clone());
        });
    }

    /// Create a vector of cloned children.
    pub fn public_children(&self, overwrite_visibility: Option<Visibility>) -> Symbols {
        self.inner
            .borrow()
            .children
            .values()
            .filter(|symbol| symbol.is_public())
            .map(|symbol| {
                if let Some(visibility) = overwrite_visibility {
                    let mut symbol = symbol.clone();
                    symbol.visibility = visibility;
                    symbol
                } else {
                    symbol.clone()
                }
            })
            .collect()
    }

    /// Clone this symbol but give the clone another visibility.
    pub fn clone_with_visibility(&self, visibility: Visibility) -> Self {
        let mut cloned = self.clone();
        cloned.visibility = visibility;
        cloned
    }

    /// Return the internal *id* of this symbol.
    pub fn id(&self) -> Identifier {
        self.inner.borrow().def.id()
    }

    /// Get any child with the given `id`.
    /// # Arguments
    /// - `id`: Anticipated *id* of the possible child.
    pub fn get(&self, id: &Identifier) -> Option<Symbol> {
        self.inner.borrow().children.get(id).cloned()
    }

    /// True if symbol has any children
    pub fn is_empty(&self) -> bool {
        self.inner.borrow().children.is_empty()
    }

    /// Return `true` if symbol's visibility is private
    pub fn visibility(&self) -> Visibility {
        self.visibility
    }

    /// Return `true` if symbol's visibility set to is public.
    pub fn is_public(&self) -> bool {
        matches!(self.visibility(), Visibility::Public)
    }

    /// Return `true` if symbol's visibility set to is non-public.
    pub fn is_private(&self) -> bool {
        !self.is_public()
    }

    /// Search down the symbol tree for a qualified name.
    /// # Arguments
    /// - `name`: Name to search for.
    pub fn search(&self, name: &QualifiedName) -> Option<Symbol> {
        log::trace!("Searching {name} in {:?}", self.full_name());
        if let Some(first) = name.first() {
            if let Some(child) = self.get(first) {
                let name = &name.remove_first();
                if name.is_empty() {
                    log::trace!("Found {name:?} in {:?}", self.full_name());
                    Some(child.clone())
                } else {
                    child.search(name)
                }
            } else {
                log::trace!("No child in {:?} while searching for {name:?}", self.id());
                None
            }
        } else {
            log::warn!("Cannot search for an anonymous name");
            None
        }
    }

    /// check if a private symbol may be declared within this symbol
    pub fn can_const(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::Module(..) | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a value on the stack may be declared within this symbol
    pub fn can_value(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::Function(..)
                | SymbolDefinition::Workbench(..)
                | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a property may be declared within this symbol
    pub fn can_prop(&self) -> bool {
        matches!(self.inner.borrow().def, SymbolDefinition::Workbench(..))
    }

    /// check if a public symbol may be declared within this symbol
    pub fn can_pub(&self) -> bool {
        self.can_const()
    }

    /// Overwrite any value in this symbol
    pub fn set_value(&self, new_value: Value) -> ResolveResult<()> {
        match &mut self.inner.borrow_mut().def {
            SymbolDefinition::Constant(_, _, value) => {
                *value = new_value;
                Ok(())
            }
            _ => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    /// Get any value of this symbol
    pub fn get_value(&self) -> ResolveResult<Value> {
        match &self.inner.borrow().def {
            SymbolDefinition::Constant(_, _, value) => Ok(value.clone()),
            _ => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    /// Return file path of top level parent source file.
    pub fn source_path(&self) -> Option<std::path::PathBuf> {
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
    pub fn set_check(&self) {
        self.inner.borrow_mut().checked = true;
    }

    /// Mark this symbol as *used*.
    pub fn set_use(&self) {
        self.inner.borrow_mut().used = true;
    }

    /// Detach from parent.
    pub fn detach(&self) {
        self.inner.borrow_mut().parent = None;
    }

    /// Get a clone of the symbol definition.
    pub fn get_def(&self) -> SymbolDefinition {
        self.inner.borrow().def.clone()
    }

    /// Work with the symbol definition.
    pub fn with_def<T>(&self, mut f: impl FnMut(&SymbolDefinition) -> T) -> T {
        f(&self.inner.borrow().def)
    }

    /// Work with the mutable symbol definition.
    pub fn with_def_mut<T>(&self, mut f: impl FnMut(&mut SymbolDefinition) -> T) -> T {
        f(&mut self.inner.borrow_mut().def)
    }

    pub(super) fn resolvable(&self) -> bool {
        matches!(
            self.inner.borrow().def,
            SymbolDefinition::SourceFile(..)
                | SymbolDefinition::Module(..)
                | SymbolDefinition::UseAll(..)
        )
    }

    /// Resolve use statements.
    pub(super) fn resolve(&self, context: &mut ResolveContext) -> ResolveResult<Symbols> {
        // collect used symbols from children and from self
        let (from_children, from_self): (Symbols, Symbols) = {
            let inner = self.inner.borrow();

            // check if we resolve recursive and collect symbols resolved from self
            let (resolve_children, from_self) = match &inner.def {
                SymbolDefinition::SourceFile(..) | SymbolDefinition::Module(..) => {
                    (true, Symbols::default())
                }
                SymbolDefinition::UseAll(visibility, name) => (
                    false,
                    match context.lookup(name) {
                        Ok(symbol) => symbol.public_children(Some(*visibility)),
                        Err(err) => {
                            if let Some(parent) = &inner.parent {
                                match context.lookup(&name.with_prefix(&parent.full_name())) {
                                    Ok(symbol) => symbol.public_children(Some(*visibility)),
                                    Err(err) => panic!("{err}"),
                                }
                            } else {
                                panic!("{err}")
                            }
                        }
                    },
                ),
                // skip non-modules
                _ => (false, Symbols::default()),
            };

            // collect symbols resolved from children
            let from_children: Symbols = if resolve_children {
                inner
                    .children
                    .values()
                    .filter(|child| child.resolvable())
                    .flat_map(|child| child.resolve(context))
                    .collect()
            } else {
                Symbols::default()
            };
            (from_children, from_self)
        };

        // add symbols collected from children to self
        self.inner.borrow_mut().children.extend(
            from_children
                .iter()
                .map(|symbol| (symbol.id(), symbol.clone())),
        );

        // remove all `UseAll` symbols
        self.inner
            .borrow_mut()
            .children
            .retain(|_, symbol| !matches!(symbol.inner.borrow().def, SymbolDefinition::UseAll(..)));

        // return symbols collected from self
        Ok(from_self)
    }

    /// check names in symbol definition
    pub fn check(&self, context: &mut ResolveContext) -> ResolveResult<()> {
        let names = match &self.inner.borrow().def {
            SymbolDefinition::SourceFile(sf) => sf.names(),
            SymbolDefinition::Module(m) => m.names(),
            SymbolDefinition::Workbench(wb) => wb.names(),
            SymbolDefinition::Function(f) => f.names(),
            SymbolDefinition::Alias(.., name) => name.into(),
            SymbolDefinition::UseAll(_, name) => name.into(),
            _ => Default::default(),
        };

        let prefix = self.module_name().clone();

        if !names.is_empty() {
            log::debug!("checking symbols:\n{names:?}");

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
        self.inner
            .borrow()
            .children
            .values()
            .try_for_each(|symbol| symbol.check(context))
    }

    fn module_name(&self) -> QualifiedName {
        let (id, is_module) = {
            let def = &self.inner.borrow().def;
            (
                def.id(),
                matches!(
                    def,
                    SymbolDefinition::Module(..) | SymbolDefinition::SourceFile(..)
                ),
            )
        };
        match is_module {
            true => {
                if let Some(parent) = &self.inner.borrow().parent {
                    parent.module_name().with_suffix(&id)
                } else {
                    QualifiedName::from_id(id)
                }
            }
            false => {
                if let Some(parent) = &self.inner.borrow().parent {
                    parent.module_name()
                } else {
                    unreachable!("root must be source file")
                }
            }
        }
    }
}

impl FullyQualify for Symbol {
    /// Get fully qualified name.
    fn full_name(&self) -> QualifiedName {
        let id = self.id();
        match &self.inner.borrow().parent {
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
            visibility: Visibility::default(),
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

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDefinition::SourceFile(source_file) => source_file.src_ref(),
            SymbolDefinition::Module(module) => module.src_ref(),
            SymbolDefinition::Workbench(workbench) => workbench.src_ref(),
            SymbolDefinition::Function(function) => function.src_ref(),
            SymbolDefinition::Builtin(_) => {
                unreachable!("builtin has no source code reference")
            }
            SymbolDefinition::Constant(_, identifier, _)
            | SymbolDefinition::Argument(identifier, _) => identifier.src_ref(),
            SymbolDefinition::Alias(_, identifier, _) => identifier.src_ref(),
            SymbolDefinition::UseAll(_, name) => name.src_ref(),
            #[cfg(test)]
            SymbolDefinition::Tester(id) => id.src_ref(),
        }
    }
}
