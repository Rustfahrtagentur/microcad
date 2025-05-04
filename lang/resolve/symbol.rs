use crate::{eval::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};
use custom_debug::Debug;

/// Symbol node
#[derive(Debug, Clone)]
pub struct SymbolInner {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent node
    #[debug(skip)]
    pub parent: Option<Symbol>,
    /// Symbol's children nodes
    pub children: SymbolMap,
}

/// Shortcut of `Rc<Cell<SymbolNode>>`
#[derive(Debug, Clone)]
pub struct Symbol(RcMut<SymbolInner>);

impl std::ops::Deref for Symbol {
    type Target = RcMut<SymbolInner>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Symbol {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// List of qualified names which can pe displayed
#[derive(Debug)]
pub struct Symbols(Vec<Symbol>);

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

impl std::ops::Deref for Symbols {
    type Target = Vec<Symbol>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<Symbol> for Symbols {
    fn from_iter<T: IntoIterator<Item = Symbol>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl Symbol {
    /// Create new symbol node without children
    pub fn new(def: SymbolDefinition, parent: Option<Symbol>) -> Self {
        Symbol(RcMut::new(SymbolInner {
            def,
            parent,
            children: Default::default(),
        }))
    }
    /// Create a symbol node for a source file.
    ///
    /// # Arguments
    /// - `source_file`: Resolved source file.
    pub fn new_source(source_file: Rc<SourceFile>) -> Symbol {
        Symbol::new(SymbolDefinition::SourceFile(source_file), None)
    }

    /// Create a symbol node of a built-in function.
    pub fn new_builtin_fn(id: Identifier, f: &'static BuiltinFunctionFn) -> Symbol {
        Symbol::new(
            SymbolDefinition::BuiltinFunction(BuiltinFunction::new(id, f)),
            None,
        )
    }

    /// Create a symbol node for a built-in module.
    pub fn new_builtin_module(id: &str, m: &'static BuiltinModuleFn) -> Symbol {
        Symbol::new(
            SymbolDefinition::BuiltinModule(BuiltinModule::new(Identifier::no_ref(id), m)),
            None,
        )
    }
    /// Create a symbol node for namespace.
    pub fn new_namespace(id: Identifier) -> Symbol {
        Symbol::new(
            SymbolDefinition::Namespace(NamespaceDefinition::new(id)),
            None,
        )
    }

    /// Create a symbol node for an external namespace.
    pub fn new_external(id: Identifier) -> Symbol {
        Symbol::new(
            SymbolDefinition::External(NamespaceDefinition::new(id)),
            None,
        )
    }

    /// Create a new build constant.
    pub fn new_constant(id: Identifier, value: Value) -> Symbol {
        Symbol::new(SymbolDefinition::Constant(id, value), None)
    }

    /// Print out symbols from that point.
    pub fn print_symbol(
        &self,
        f: &mut std::fmt::Formatter,
        id: Option<&Identifier>,
        depth: usize,
    ) -> std::fmt::Result {
        let self_id = &self.id();
        let id = id.unwrap_or(self_id);
        writeln!(
            f,
            "{:depth$}{id} {} [{}]",
            "",
            self.0.borrow().def,
            self.full_name()
        )?;
        let indent = 4; //format!("{id}").len();

        self.borrow()
            .children
            .iter()
            .try_for_each(|(id, child)| child.print_symbol(f, Some(id), depth + indent))
    }

    /// Insert child and change parent of child to new parent.
    pub fn insert_child(parent: &Symbol, child: Symbol) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.id();
        parent.borrow_mut().children.insert(id, child);
    }

    /// Move all children from one node to another (resets the parent of the children).
    pub fn move_children(to: &Symbol, from: &Symbol) {
        // copy children
        from.borrow().children.iter().for_each(|(id, child)| {
            child.borrow_mut().parent = Some(to.clone());
            to.borrow_mut().children.insert(id.clone(), child.clone());
        });
    }

    /// Convert the symbol definition from external to namespace
    pub fn external_to_namespace(&self) {
        let def = match &self.borrow().def {
            SymbolDefinition::External(e) => SymbolDefinition::Namespace(e.clone()),
            def => def.clone(),
        };
        self.borrow_mut().def = def
    }

    /// Get id of the definition in this node.
    pub fn id(&self) -> Identifier {
        self.borrow().def.id()
    }

    /// Fetch child node with an id.
    pub fn get(&self, id: &Identifier) -> Option<Symbol> {
        self.borrow().children.get(id).cloned()
    }

    /// Search down the symbol tree for a qualified name.
    pub fn search(&self, name: &QualifiedName) -> Option<Symbol> {
        log::trace!("Searching {name} in {}", self.id());
        if let Some(first) = name.first() {
            if let Some(child) = self.get(first) {
                let name = &name.remove_first();
                if name.is_empty() {
                    Some(child.clone())
                } else {
                    child.search(name)
                }
            } else {
                log::debug!("No child in {} while searching for {name}", self.id());
                None
            }
        } else {
            log::warn!("Cannot search for an anonymous name");
            None
        }
    }

    /// Returns if symbol is an external namespace which must be loaded before using.
    pub fn is_external(&self) -> bool {
        matches!(&self.borrow().def, SymbolDefinition::External(_))
    }

    /// True if symbol has any children
    pub fn is_empty(&self) -> bool {
        self.borrow().children.is_empty()
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
            None => QualifiedName(vec![id]),
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
            SymbolDefinition::Namespace(namespace) | SymbolDefinition::External(namespace) => {
                namespace.src_ref()
            }
            SymbolDefinition::Module(module) => module.src_ref(),
            SymbolDefinition::Function(function) => function.src_ref(),
            SymbolDefinition::BuiltinFunction(_) | SymbolDefinition::BuiltinModule(_) => {
                unreachable!("builtin has no source code reference")
            }
            SymbolDefinition::Constant(identifier, value) => SrcRef::merge(identifier, value),
            SymbolDefinition::Alias(identifier, name) => SrcRef::merge(identifier, name),
        }
    }
}
