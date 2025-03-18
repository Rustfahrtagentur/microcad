/// Source File `foo.µcad`
///
/// module a() {
///     b = 42.0;
///     function bar() { 13 }
/// }
/// namespace c { function d() { 23 } }
///
/// Symbol Tree example:
/// foo.µcad
///     ModuleDefinition(a)
///         FunctionDefinition(bar)
///         Statements
///             b
///     NamespaceDefinition(c)
///         d
///
/// Usage:
///
/// foo = a();
/// print("{foo.b}"); // 42.0
///
/// v = c::d();

enum SymbolDefinition {
    SourceFile(std::rc::Rc<SourceFile>),
    Namespace(std::rc::Rc<NamespaceDefinition>),
    Module(std::rc::Rc<ModuleDefinition>),
    Function(std::rc::Rc<FunctionDefinition>),
}

impl SymbolDefinition {
    fn id(&self) -> Id {
        match &self {
            Self::Namespace(n) => n.name.id().clone(),
            Self::Module(m) => m.name.id().clone(),
            _ => unimplemented!(),
        }
    }
}

//pub type SymbolNode = rctree::Node<SymbolNodeInner>;
struct SymbolNode {
    def: SymbolDefinition,
    children: std::collections::btree_map::BTreeMap<Id, std::rc::Rc<SymbolNode>>,
}

trait Resolve {
    fn resolve(&self) -> SymbolNode;
}

impl Statement {
    fn definition(&self) -> Option<SymbolDefinition> {
        match &self {
            Statement::NamespaceDefinition(n) => Some(SymbolDefinition::Namespace(n.clone())),
            Statement::ModuleDefinition(m) => Some(SymbolDefinition::Module(m.clone())),
            _ => None,
        }
    }
}

impl Resolve for std::rc::Rc<ModuleDefinition> {
    fn resolve(&self) -> SymbolNode {
        let mut node = SymbolNode {
            def: SymbolDefinition::Module(self.clone()),
            children: std::collections::btree_map::BTreeMap::new(),
        };

        for statement in self
            .body
            .pre_init_statements
            .iter()
            .chain(&self.body.post_init_statements)
        // TODO put this into ModuleDefinition::statements()
        {
            match statement {
                ModuleDefinitionStatement::ModuleDefinition(m) => {
                    node.children
                        .insert(m.name.id().clone(), std::rc::Rc::new(m.resolve()));
                }
                // TODO Function definition
                _ => unimplemented!(),
            }
        }

        node
    }
}

impl Resolve for std::rc::Rc<NamespaceDefinition> {
    fn resolve(&self) -> SymbolNode {
        let mut node = SymbolNode {
            def: SymbolDefinition::Module(self.clone()),
            children: std::collections::btree_map::BTreeMap::new(),
        };
    }
}

impl Resolve for SymbolDefinition {
    fn resolve(&self) -> SymbolNode {
        match &self {
            Self::Module(m) => m.resolve(),
            Self::Namespace(n) => n.resolve(),
            _ => unimplemented!(),
        }
    }
}

impl Resolve for std::rc::Rc<SourceFile> {
    fn resolve(&self) -> SymbolNode {
        let mut node = SymbolNode {
            def: SymbolDefinition::SourceFile(self.clone()),
            children: std::collections::btree_map::BTreeMap::new(),
        };

        for statement in &self.body {
            match statement.definition() {
                Some(def) => {
                    node.children
                        .insert(def.id(), std::rc::Rc::new(def.resolve()));
                }
                None => {}
            }
        }

        node
    }
}
