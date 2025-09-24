// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{builtin::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};
use custom_debug::Debug;
use derive_more::{Deref, DerefMut};

/// Symbol content
#[derive(Debug, Clone)]
pub struct SymbolInner {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent
    #[debug(skip)]
    pub parent: Option<Symbol>,
    /// Symbol's children
    pub children: SymbolMap,
    /// Flag if this symbol was in use
    pub used: bool,
}

impl Default for SymbolInner {
    fn default() -> Self {
        Self {
            def: SymbolDefinition::SourceFile(SourceFile::default().into()),
            parent: Default::default(),
            children: Default::default(),
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
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Symbol {
    visibility: Visibility,
    #[deref]
    #[deref_mut]
    inner: RcMut<SymbolInner>,
}

/// List of qualified names which can pe displayed
#[derive(Debug, Deref)]
pub struct Symbols(Vec<Symbol>);

impl Symbols {
    /// Return all fully qualified names of all symbols.
    pub fn full_names(&self) -> QualifiedNames {
        self.iter().map(|symbol| symbol.full_name()).collect()
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
        // just compare the visibility and the pointers - not the content
        self.visibility == other.visibility && self.inner.as_ptr() == other.inner.as_ptr()
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

    /// Create a new argument ([`SymbolDefinition::Argument`]).
    pub fn new_call_argument(id: Identifier, value: Value) -> Symbol {
        Symbol::new(SymbolDefinition::Argument(id, value), None)
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
        children: bool,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        if cfg!(feature = "ansi-color") && !self.borrow().used {
            color_print::cwrite!(
                f,
                "{:depth$}<#606060>{visibility}{id:?} {def} [{full_name}]</>",
                "",
                visibility = self.visibility(),
                def = self.inner.borrow().def,
                full_name = self.full_name(),
            )?;
        } else {
            write!(
                f,
                "{:depth$}{id:?} {} [{}]",
                "",
                self.inner.borrow().def,
                self.full_name(),
            )?;
        }
        if children {
            writeln!(f)?;
            let indent = 4;

            self.borrow().children.iter().try_for_each(|(id, child)| {
                child.print_symbol(f, Some(id), depth + indent, true)
            })?;
        }
        Ok(())
    }

    /// Insert child and change parent of child to new parent.
    /// # Arguments
    /// - `parent`: New parent symbol (will be changed in child!).
    /// - `child`: Child to insert
    pub fn add_child(parent: &Symbol, child: Symbol) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.id();
        parent.borrow_mut().children.insert(id, child);
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

    pub fn set_children(&self, new_children: SymbolMap) {
        assert!(self.borrow().children.is_empty());
        self.borrow_mut().children = new_children;
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
        from.borrow().children.iter().for_each(|(id, child)| {
            child.borrow_mut().parent = Some(self.clone());
            self.borrow_mut().children.insert(id.clone(), child.clone());
        });
    }

    /// Clone this symbol but give the clone another visibility.
    pub fn clone_with_visibility(&self, visibility: Visibility) -> Self {
        let mut cloned = self.clone();
        cloned.visibility = visibility;
        cloned
    }

    /// Return the internal *id* of this symbol.
    pub fn id(&self) -> Identifier {
        self.borrow().def.id()
    }

    /// Get any child with the given `id`.
    /// # Arguments
    /// - `id`: Anticipated *id* of the possible child.
    pub fn get(&self, id: &Identifier) -> Option<Symbol> {
        self.borrow().children.get(id).cloned()
    }

    /// True if symbol has any children
    pub fn is_empty(&self) -> bool {
        self.borrow().children.is_empty()
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
            self.borrow().def,
            SymbolDefinition::Module(..) | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a value on the stack may be declared within this symbol
    pub fn can_value(&self) -> bool {
        matches!(
            self.borrow().def,
            SymbolDefinition::Function(..)
                | SymbolDefinition::Workbench(..)
                | SymbolDefinition::SourceFile(..)
        )
    }

    /// check if a property may be declared within this symbol
    pub fn can_prop(&self) -> bool {
        matches!(self.borrow().def, SymbolDefinition::Workbench(..))
    }

    /// check if a public symbol may be declared within this symbol
    pub fn can_pub(&self) -> bool {
        self.can_const()
    }

    /// Overwrite any value in this symbol
    pub fn set_value(&self, new_value: Value) -> ResolveResult<()> {
        match &mut self.borrow_mut().def {
            SymbolDefinition::Constant(_, _, value) => {
                *value = new_value;
                Ok(())
            }
            _ => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    /// Get any value of this symbol
    pub fn get_value(&self) -> ResolveResult<Value> {
        match &self.borrow().def {
            SymbolDefinition::Constant(_, _, value) => Ok(value.clone()),
            _ => Err(ResolveError::NotAValue(self.full_name())),
        }
    }

    pub fn current_path(&self) -> Option<std::path::PathBuf> {
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
            .and_then(|parent| parent.current_path())
    }
}

impl FullyQualify for Symbol {
    /// Get fully qualified name.
    fn full_name(&self) -> QualifiedName {
        let id = self.id();
        match &self.borrow().parent {
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

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, None, 0, false)
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
