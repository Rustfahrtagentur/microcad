//! Namespace body parser entity

use crate::{errors::*, eval::*, parse::*, parser::*, src_ref::*};

/// Namespace body
#[derive(Debug, Default)]
pub struct NamespaceBody {
    /// Namespace statements
    pub statements: Vec<NamespaceStatement>,
    /// Namespace's local symbol table
    pub symbols: SymbolTable,
    /// Source code reference
    src_ref: SrcRef,
}

impl NamespaceBody {
    /// Add statement to namespace
    pub fn add_statement(&mut self, statement: NamespaceStatement) {
        self.statements.push(statement.clone());
        match statement {
            NamespaceStatement::FunctionDefinition(function) => {
                self.add_function(function);
            }
            NamespaceStatement::NamespaceDefinition(namespace) => {
                self.add_namespace(namespace);
            }
            NamespaceStatement::ModuleDefinition(module) => {
                self.add_module(module);
            }
            _ => {}
        }
    }
}

impl Symbols for NamespaceBody {
    fn find_symbols(&self, id: &Id) -> Vec<&Symbol> {
        self.symbols.find_symbols(id)
    }

    fn add_symbol(&mut self, symbol: Symbol) -> &mut Self {
        self.symbols.add_symbol(symbol);
        self
    }

    fn copy_symbols<T: Symbols>(&self, into: &mut T) {
        self.symbols.copy_symbols(into)
    }
}

impl Parse for NamespaceBody {
    fn parse(pair: Pair<'_>) -> ParseResult<Self> {
        let mut body = NamespaceBody::default();
        for pair in pair.clone().into_inner() {
            match pair.as_rule() {
                Rule::namespace_statement => { body.add_statement(NamespaceStatement::parse(pair)?); }
                _ => {}
            }
        }

        body.src_ref = pair.into();

        Ok(body)
    }
}

impl SrcReferrer for NamespaceBody {
    fn src_ref(&self) -> SrcRef {
        self.src_ref.clone()
    }
}

impl Eval for NamespaceBody {
    type Output = ();

    fn eval(&self, context: &mut Context) -> Result<Self::Output> {
        for statement in &self.statements {
            statement.eval(context)?;
        }

        Ok(())
    }
}