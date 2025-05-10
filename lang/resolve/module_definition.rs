use crate::{rc::*, resolve::*, syntax::*};

impl ModuleDefinition {
    /// Resolve into SymbolNode
    pub fn resolve(self: &Rc<Self>, parent: Option<Symbol>) -> Symbol {
        let node = Symbol::new(SymbolDefinition::Module(self.clone()), parent);
        node.borrow_mut().children = self.body.resolve(Some(node.clone()));
        node
    }
}
