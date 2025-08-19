// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};
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

/// Symbol
///
/// Every `Symbol` has a [`SymbolDefinition`], a *parent* and *children* stored within a `Rc<RefCell<`[`SymbolInner`]`>`.
/// So `Symbol` is meant as a tree which is used by [`SymbolTable`] to store
/// the resolved symbols by it's original structure in the source code and by it's *id*.
///
/// `Symbol` can be shared as mutable.
#[derive(Debug, Clone, Deref, DerefMut)]
pub struct Symbol(RcMut<SymbolInner>);

/// List of qualified names which can pe displayed
#[derive(Debug, Deref)]
pub struct Symbols(Vec<Symbol>);

impl Default for Symbol {
    fn default() -> Self {
        Self(RcMut::new(SymbolInner {
            def: SymbolDefinition::SourceFile(SourceFile::default().into()),
            parent: Default::default(),
            children: Default::default(),
            used: false,
        }))
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
    /// - `def`: Symbol definition
    /// - `parent`: Symbol's parent symbol or none for root
    pub fn new(def: SymbolDefinition, parent: Option<Symbol>) -> Self {
        Symbol(RcMut::new(SymbolInner {
            def,
            parent,
            children: Default::default(),
            used: false,
        }))
    }
    /// Create a symbol of a source file ([`SymbolDefinition::SourceFile`]).
    /// # Arguments
    /// - `source_file`: Resolved source file.
    pub fn new_source(source_file: Rc<SourceFile>) -> Symbol {
        Symbol::new(SymbolDefinition::SourceFile(source_file), None)
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

    /// Create a symbol for module.
    /// # Arguments
    /// - `id`: Name of the symbol
    pub fn new_module(id: Identifier) -> Symbol {
        Symbol::new(SymbolDefinition::Module(ModuleDefinition::new(id)), None)
    }

    /// Create a symbol for an external  ([`SymbolDefinition::External`])..
    /// # Arguments
    /// - `id`: Name of the symbol
    pub fn new_external(id: Identifier) -> Symbol {
        Symbol::new(SymbolDefinition::External(ModuleDefinition::new(id)), None)
    }

    /// Create a new constant ([`SymbolDefinition::Constant`]).
    /// # Arguments
    /// - `id`: Name of the symbol
    /// - `value`: The value to store
    pub fn new_constant(id: Identifier, value: Value) -> Symbol {
        Symbol::new(SymbolDefinition::Constant(id, value), None)
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
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        if cfg!(feature = "ansi-color") && !self.borrow().used {
            color_print::cwriteln!(
                f,
                "{:depth$}<#606060>{id:?} {} [{}]</>",
                "",
                self.0.borrow().def,
                self.full_name(),
            )?;
        } else {
            writeln!(
                f,
                "{:depth$}{id:?} {} [{}]",
                "",
                self.0.borrow().def,
                self.full_name(),
            )?;
        }
        let indent = 4; //format!("{id}").len();

        self.borrow()
            .children
            .iter()
            .try_for_each(|(id, child)| child.print_symbol(f, Some(id), depth + indent))
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

    /// Search down the symbol tree for a qualified name.
    /// # Arguments
    /// - `name`: Name to search for.
    pub fn search(&self, name: &QualifiedName) -> Option<Symbol> {
        log::trace!("Searching {name} in {:?}", self.id());
        if let Some(first) = name.first() {
            if let Some(child) = self.get(first) {
                let name = &name.remove_first();
                if name.is_empty() {
                    Some(child.clone())
                } else {
                    child.search(name)
                }
            } else {
                log::trace!("No child in {:?} while searching for {name}", self.id());
                None
            }
        } else {
            log::warn!("Cannot search for an anonymous name");
            None
        }
    }

    /// Converts the *symbol definition* from [`SymbolDefinition::External`] into [`SymbolDefinition::Module`]
    /// without changing the inner [`ModuleDefinition`].
    ///
    /// Symbols which have not already been loaded from [`Externals`] into [`SourceCache`] will remain
    /// of type [`SymbolDefinition::External`] until they get loaded.
    pub fn external_to_module(&self) {
        let def = match &self.borrow().def {
            SymbolDefinition::External(e) => SymbolDefinition::Module(e.clone()),
            def => def.clone(),
        };
        self.borrow_mut().def = def
    }

    /// Returns if symbol is an external module which must be loaded before using.
    pub fn is_external(&self) -> bool {
        matches!(&self.borrow().def, SymbolDefinition::External(_))
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
        self.print_symbol(f, None, 0)
    }
}

/// Print symbols via [std::fmt::Display]
pub struct FormatSymbol<'a>(pub &'a Symbol);

impl std::fmt::Display for FormatSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_symbol(f, Some(&self.0.id()), 0)
    }
}

impl SrcReferrer for SymbolInner {
    fn src_ref(&self) -> SrcRef {
        match &self.def {
            SymbolDefinition::SourceFile(source_file) => source_file.src_ref(),
            SymbolDefinition::Module(module) | SymbolDefinition::External(module) => {
                module.src_ref()
            }
            SymbolDefinition::Workbench(workbench) => workbench.src_ref(),
            SymbolDefinition::Function(function) => function.src_ref(),
            SymbolDefinition::Builtin(_) => {
                unreachable!("builtin has no source code reference")
            }
            SymbolDefinition::Constant(identifier, _)
            | SymbolDefinition::Argument(identifier, _) => identifier.src_ref(),
            SymbolDefinition::Alias(identifier, _) => identifier.src_ref(),
            SymbolDefinition::UseAll(name) => name.src_ref(),
        }
    }
}
