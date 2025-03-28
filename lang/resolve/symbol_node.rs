use crate::{eval::*, rc_mut::*, resolve::*, syntax::*, value::*, Id};

/// Symbol node
#[derive(Debug)]
pub struct SymbolNode {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent node
    pub parent: Option<RcMut<SymbolNode>>,
    /// Symbol's children nodes
    pub children: SymbolMap,
}

impl SymbolNode {
    /// Create new reference counted symbol node
    pub fn new(def: SymbolDefinition, parent: Option<RcMut<SymbolNode>>) -> RcMut<SymbolNode> {
        RcMut::new(SymbolNode {
            def,
            parent,
            children: Default::default(),
        })
    }

    /// Create a symbol node of a built-in function
    pub fn new_builtin_fn(id: Id, f: &'static BuiltinFunctionFn) -> RcMut<SymbolNode> {
        SymbolNode::new(
            SymbolDefinition::BuiltinFunction(BuiltinFunction::new(id, f)),
            None,
        )
    }

    /// Create a symbol node for a built-in module
    pub fn new_builtin_module(id: &str, m: &'static BuiltinModuleFn) -> RcMut<SymbolNode> {
        SymbolNode::new(
            SymbolDefinition::BuiltinModule(BuiltinModule::new(id.into(), m)),
            None,
        )
    }

    /// Create a symbol node for namespace
    pub fn new_builtin_namespace(id: &str) -> RcMut<SymbolNode> {
        SymbolNode::new(
            SymbolDefinition::Namespace(NamespaceDefinition::new(id.into())),
            None,
        )
    }

    /// Create a new build constant
    pub fn new_builtin_constant(id: &str, value: Value) -> RcMut<SymbolNode> {
        SymbolNode::new(SymbolDefinition::Constant(id.into(), value), None)
    }

    /// Print out symbols from that point
    pub fn print_symbol(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}{}", "", self.def)?;
        self.children
            .iter()
            .try_for_each(|(_, child)| child.borrow().print_symbol(f, depth + 1))
    }

    /// Insert child and change parent of child to new parent
    pub fn insert_child(parent: &mut RcMut<SymbolNode>, child: RcMut<SymbolNode>) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.borrow().def.id();
        parent.borrow_mut().children.insert(id, child);
    }

    /// Fetch child node with an id
    pub fn fetch(&self, id: &Id) -> Option<&RcMut<SymbolNode>> {
        self.children.get(id)
    }

    /// Search in symbol tree by a path, e.g. a::b::c
    pub fn search_down(&self, path: &SymbolPath) -> Option<RcMut<SymbolNode>> {
        if let Some(first) = path.first() {
            if let Some(child) = self.fetch(first) {
                let path = &path.remove_first();
                if path.is_empty() {
                    Some(child.clone())
                } else {
                    child.borrow().search_down(path)
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Search for first symbol in parents
    pub fn search_up(&self, path: &SymbolPath) -> Option<RcMut<SymbolNode>> {
        if let Some(child) = self.search_down(path) {
            Some(child)
        } else if let Some(parent) = &self.parent {
            if let Some(child) = parent.borrow().search_down(path) {
                Some(child.clone())
            } else {
                parent.borrow().search_up(path)
            }
        } else {
            None
        }
    }
}

impl std::fmt::Display for SymbolNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_symbol(f, 0)
    }
}

/// print symbols via std::fmt::Display
pub struct FormatSymbol<'a>(pub &'a SymbolNode);

impl std::fmt::Display for FormatSymbol<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.print_symbol(f, 0)
    }
}

/// Map Id to SymbolNode reference
pub type SymbolMap = std::collections::btree_map::BTreeMap<Id, RcMut<SymbolNode>>;
