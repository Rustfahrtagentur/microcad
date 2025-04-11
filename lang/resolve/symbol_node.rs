use crate::{eval::*, rc_mut::*, resolve::*, src_ref::*, syntax::*, value::*, Id};

use custom_debug::Debug;

/// Symbol node
#[derive(Debug)]
pub struct SymbolNode {
    /// Symbol definition
    pub def: SymbolDefinition,
    /// Symbol's parent node
    #[debug(skip)]
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
    pub fn new_namespace(id: Identifier) -> RcMut<SymbolNode> {
        SymbolNode::new(
            SymbolDefinition::Namespace(NamespaceDefinition::new(id)),
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
            .try_for_each(|(_, child)| child.borrow().print_symbol(f, depth + self.def.id().len()))
    }

    /// Insert child and change parent of child to new parent
    pub fn insert_child(parent: &RcMut<SymbolNode>, child: RcMut<SymbolNode>) {
        child.borrow_mut().parent = Some(parent.clone());
        let id = child.borrow().def.id();
        parent.borrow_mut().children.insert(id, child);
    }

    /// Get id of the definition in this node
    pub fn id(&self) -> Id {
        self.def.id()
    }

    /// Get fully qualified name
    pub fn name(&self) -> EvalResult<QualifiedName> {
        let id = Identifier(Refer::none(self.id()));
        match &self.parent {
            Some(parent) => {
                let mut name = parent.borrow().name()?.clone();
                name.push(id);
                Ok(name)
            }
            None => Ok(QualifiedName(vec![id])),
        }
    }

    /// Insert a child at the correct sub node
    pub fn insert(&mut self, name: &QualifiedName, node: RcMut<SymbolNode>) {
        eprintln!("SymbolNode: Insert {name} into {}", self.def.id());
        let (first, name) = name.split_first();
        if !name.is_empty() {
            if let Some(child) = self.children.get(first.id()) {
                child.borrow_mut().insert(&name, node);
                return;
            }
        }
        self.children.insert(first.id().clone(), node);
    }

    /// Fetch child node with an id
    pub fn get(&self, id: &Id) -> Option<&RcMut<SymbolNode>> {
        self.children.get(id)
    }

    /// Search in symbol tree by a path, e.g. a::b::c
    pub fn search_down(&self, name: &QualifiedName) -> Option<RcMut<SymbolNode>> {
        eprintln!("search_down {name} in {}", self.id());
        if let Some(first) = name.first() {
            if let Some(child) = self.get(first.id()) {
                let name = &name.remove_first();
                if name.is_empty() {
                    Some(child.clone())
                } else {
                    child.borrow().search_down(name)
                }
            } else {
                eprintln!("search_down no child in {}", self.id());
                None
            }
        } else {
            eprintln!("search_down no first in {name}");
            None
        }
    }

    /// Search for first symbol in parents
    pub fn search_up(&self, name: &QualifiedName) -> Option<RcMut<SymbolNode>> {
        if let Some(result) = self.search_down(name) {
            Some(result)
        } else if let Some(parent) = &self.parent {
            if let Some(result) = parent.borrow().search_down(name) {
                Some(result.clone())
            } else {
                parent.borrow().search_up(name)
            }
        } else {
            None
        }
    }
}

impl Eval for SymbolNode {
    fn eval(&self, context: &mut EvalContext) -> EvalResult<Value> {
        match &self.def {
            SymbolDefinition::SourceFile(s) => s.eval(context),
            _ => todo!(),
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
