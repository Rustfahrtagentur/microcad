// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Collect names from within definitions.

use crate::syntax::*;

pub(super) trait Names {
    /// Return list of qualified names which occur in self.
    fn names(&self) -> Vec<&QualifiedName>;
}

impl Names for SourceFile {
    fn names(&self) -> Vec<&QualifiedName> {
        self.statements.names()
    }
}

impl Names for ModuleDefinition {
    fn names(&self) -> Vec<&QualifiedName> {
        if let Some(body) = &self.body {
            body.names()
        } else {
            Vec::new()
        }
    }
}

impl Names for StatementList {
    fn names(&self) -> Vec<&QualifiedName> {
        self.iter()
            .flat_map(|statement| statement.names())
            .collect()
    }
}

impl Names for Statement {
    fn names(&self) -> Vec<&QualifiedName> {
        match self {
            // avoid statements which get symbolized or have no symbol names
            Statement::Workbench(_)
            | Statement::Module(_)
            | Statement::Function(_)
            | Statement::InnerAttribute(_) => Vec::new(),

            Statement::Init(i) => i.names(),
            Statement::Use(u) => u.names(),
            Statement::Return(r) => r.names(),
            Statement::If(i) => i.names(),
            Statement::Assignment(a) => a.names(),
            Statement::Expression(e) => e.names(),
        }
    }
}

impl Names for WorkbenchDefinition {
    fn names(&self) -> Vec<&QualifiedName> {
        let mut names = self.body.names();
        names.append(&mut self.plan.names());
        names
    }
}

impl Names for ParameterList {
    fn names(&self) -> Vec<&QualifiedName> {
        self.iter()
            .filter_map(|param| param.default_value.as_ref())
            .flat_map(|expr| expr.names())
            .collect()
    }
}

impl Names for FunctionDefinition {
    fn names(&self) -> Vec<&QualifiedName> {
        let mut names = self.body.names();
        names.append(&mut self.signature.names());
        names
    }
}

impl Names for FunctionSignature {
    fn names(&self) -> Vec<&QualifiedName> {
        self.parameters.names()
    }
}

impl Names for InitDefinition {
    fn names(&self) -> Vec<&QualifiedName> {
        let mut names = self.body.names();
        names.append(&mut self.parameters.names());
        names
    }
}

impl Names for ReturnStatement {
    fn names(&self) -> Vec<&QualifiedName> {
        if let Some(result) = &self.result {
            result.names()
        } else {
            Vec::new()
        }
    }
}

impl Names for IfStatement {
    fn names(&self) -> Vec<&QualifiedName> {
        todo!()
    }
}

impl Names for AssignmentStatement {
    fn names(&self) -> Vec<&QualifiedName> {
        self.assignment.expression.names()
    }
}

impl Names for ExpressionStatement {
    fn names(&self) -> Vec<&QualifiedName> {
        self.expression.names()
    }
}

impl Names for Expression {
    fn names(&self) -> Vec<&QualifiedName> {
        match self {
            Expression::Invalid
            | Expression::Value(_)
            | Expression::Literal(_)
            | Expression::Marker(_) => Vec::new(),

            Expression::FormatString(fs) => fs.names(),
            Expression::ArrayExpression(ae) => ae.names(),
            Expression::TupleExpression(te) => te.names(),
            Expression::Body(body) => body.names(),
            Expression::Call(call) => call.names(),
            Expression::QualifiedName(name) => vec![name],
            Expression::BinaryOp {
                lhs, op: _, rhs, ..
            } => {
                let mut names = lhs.names();
                names.append(&mut rhs.names());
                names
            }
            Expression::UnaryOp { op: _, rhs, .. } => rhs.names(),
            Expression::ArrayElementAccess(e, e1, ..) => {
                let mut names = e.names();
                names.append(&mut e1.names());
                names
            }
            Expression::PropertyAccess(e, ..) => e.names(),
            Expression::AttributeAccess(e, ..) => e.names(),
            Expression::MethodCall(e, mc, ..) => {
                let mut names = e.names();
                names.append(&mut mc.names());
                names
            }
        }
    }
}

impl Names for Body {
    fn names(&self) -> Vec<&QualifiedName> {
        todo!()
    }
}

impl Names for UseStatement {
    fn names(&self) -> Vec<&QualifiedName> {
        match &self.decl {
            UseDeclaration::Use(name)
            | UseDeclaration::UseAll(name)
            | UseDeclaration::UseAlias(name, _) => vec![name],
        }
    }
}

impl Names for FormatString {
    fn names(&self) -> Vec<&QualifiedName> {
        todo!()
    }
}

impl Names for ArrayExpression {
    fn names(&self) -> Vec<&QualifiedName> {
        todo!()
    }
}

impl Names for TupleExpression {
    fn names(&self) -> Vec<&QualifiedName> {
        todo!()
    }
}

impl Names for Call {
    fn names(&self) -> Vec<&QualifiedName> {
        let mut names = self.argument_list.names();
        names.push(&self.name);
        names
    }
}

impl Names for ArgumentList {
    fn names(&self) -> Vec<&QualifiedName> {
        self.iter().flat_map(|arg| arg.value.names()).collect()
    }
}

impl Names for MethodCall {
    fn names(&self) -> Vec<&QualifiedName> {
        let mut names = self.argument_list.names();
        names.push(&self.name);
        names
    }
}
