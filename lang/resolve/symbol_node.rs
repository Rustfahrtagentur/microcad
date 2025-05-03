use crate::{eval::*, rc::*, resolve::*, src_ref::*, syntax::*, value::*};
use custom_debug::Debug;

/// Symbol node
#[derive(Debug, Clone)]
pub struct SymbolNode {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent node
    #[debug(skip)]
    pub parent: Option<SymbolNodeRcMut>,
    /// Symbol's children nodes
    pub children: SymbolMap,
}

/// Shortcut of `Rc<Cell<SymbolNode>>`
pub type SymbolNodeRcMut = RcMut<SymbolNode>;

/// List of qualified names which can pe displayed
#[derive(Debug)]
pub struct SymbolNodes(Vec<SymbolNodeRcMut>);

impl std::fmt::Display for SymbolNodes {
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

impl std::ops::Deref for SymbolNodes {
    type Target = Vec<SymbolNodeRcMut>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<SymbolNodeRcMut> for SymbolNodes {
    fn from_iter<T: IntoIterator<Item = SymbolNodeRcMut>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl SymbolNode {
    /// Create a symbol node for a source file.
    ///
    /// # Arguments
    /// - `source_file`: Resolved source file.
    pub fn new_source(source_file: Rc<SourceFile>) -> SymbolNodeRcMut {
        SymbolNode::new(SymbolDefinition::SourceFile(source_file), None)
    }

    /// Create new reference counted symbol node.
    pub fn new(def: SymbolDefinition, parent: Option<SymbolNodeRcMut>) -> SymbolNodeRcMut {
        RcMut::new(SymbolNode {
            def,
            parent,
            children: Default::default(),
        })
    }

    /// Create a symbol node of a built-in function.
    pub fn new_builtin_fn(id: Identifier, f: &'static BuiltinFunctionFn) -> SymbolNodeRcMut {
        SymbolNode::new(
            SymbolDefinition::BuiltinFunction(BuiltinFunction::new(id, f)),
            None,
        )
    }

    /// Create a symbol node for a built-in module.
    pub fn new_builtin_module(id: &str, m: &'static BuiltinModuleFn) -> SymbolNodeRcMut {
        SymbolNode::new(
            SymbolDefinition::BuiltinModule(BuiltinModule::new(
                Identifier(Refer::none(id.into())),
                m,
            )),
            None,
        )
    }

    /// Create a symbol node for namespace.
    pub fn new_namespace(id: Identifier) -> SymbolNodeRcMut {
        SymbolNode::new(
            SymbolDefinition::Namespace(NamespaceDefinition::new(id)),
            None,
        )
    }

    /// Create a symbol node for an external namespace.
    pub fn new_external(id: Identifier) -> SymbolNodeRcMut {
        SymbolNode::new(
            SymbolDefinition::External(NamespaceDefinition::new(id)),
            None,
        )
    }

    /// Create a new build constant.
    pub fn new_constant(id: Identifier, value: Value) -> SymbolNodeRcMut {
        SymbolNode::new(SymbolDefinition::Constant(id, value), None)
    }

    /// Print out symbols from that point.
    pub fn print_symbol(
        &self,
        f: &mut std::fmt::Formatter,
        id: Option<&Identifier>,
        depth: usize,
    ) -> std::fmt::Result {
        let self_id = &self.def.id();
        let id = id.unwrap_or(self_id);
        writeln!(f, "{:depth$}{id} {} [{}]", "", self.def, self.full_name())?;
        let indent = format!("{id}").len();

        self.children
            .iter()
            .try_for_each(|(id, child)| child.borrow().print_symbol(f, Some(id), depth + indent))
    }

    /// Insert child and change parent of child to new parent.
    pub fn insert_child(parent: &SymbolNodeRcMut, child: SymbolNodeRcMut) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.borrow().id();
        parent.borrow_mut().children.insert(id, child);
    }

    /// Move all children from one node to another (resets the parent of the children).
    pub fn move_children(to: &SymbolNodeRcMut, from: &SymbolNodeRcMut) {
        // copy children
        from.borrow().children.iter().for_each(|(id, child)| {
            child.borrow_mut().parent = Some(to.clone());
            to.borrow_mut().children.insert(id.clone(), child.clone());
        });
    }

    /// Convert the symbol definition from external to namespace
    pub fn external_to_namespace(&mut self) {
        self.def = match &self.def {
            SymbolDefinition::External(e) => SymbolDefinition::Namespace(e.clone()),
            def => def.clone(),
        }
    }

    /// Get id of the definition in this node.
    pub fn id(&self) -> Identifier {
        self.def.id()
    }

    /// Fetch child node with an id.
    pub fn get(&self, id: &Identifier) -> Option<&SymbolNodeRcMut> {
        self.children.get(id)
    }

    /// Search down the symbol tree for a qualified name.
    pub fn search(&self, name: &QualifiedName) -> Option<SymbolNodeRcMut> {
        log::trace!("Searching {name} in {}", self.id());
        if let Some(first) = name.first() {
            if let Some(child) = self.get(first) {
                let name = &name.remove_first();
                if name.is_empty() {
                    Some(child.clone())
                } else {
                    child.borrow().search(name)
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
        matches!(&self.def, SymbolDefinition::External(_))
    }
}

impl FullyQualify for SymbolNode {
    /// Get fully qualified name.
    fn full_name(&self) -> QualifiedName {
        let id = self.id();
        match &self.parent {
            Some(parent) => {
                let mut name = parent.borrow().full_name();
                name.push(id);
                name
            }
            None => QualifiedName(vec![id]),
        }
    }
}

impl std::fmt::Display for SymbolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, None, 0)
    }
}

impl std::fmt::Display for SymbolNodeRcMut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.borrow().print_symbol(f, None, 0)
    }
}

/// Print symbols via [std::fmt::Display]
pub struct FormatSymbol<'a>(pub &'a SymbolNode);

impl std::fmt::Display for FormatSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_symbol(f, Some(&self.0.id()), 0)
    }
}

impl SrcReferrer for SymbolNode {
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
