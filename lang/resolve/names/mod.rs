// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Collect names from within definitions.

mod name_list;

use crate::syntax::*;
use name_list::*;

pub(super) trait Names {
    fn names(&self) -> NameList;
}

impl Names for SourceFile {
    fn names(&self) -> NameList {
        self.statements.names().drop_locals()
    }
}

impl Names for ModuleDefinition {
    fn names(&self) -> NameList {
        if let Some(body) = &self.body {
            body.names().drop_locals()
        } else {
            Default::default()
        }
    }
}

impl Names for StatementList {
    fn names(&self) -> NameList {
        let mut names = NameList::default();
        self.iter()
            .for_each(|statement| names.merge_in_place(statement.names()));
        names
    }
}

impl Names for Statement {
    fn names(&self) -> NameList {
        match self {
            Statement::Workbench(wd) => NameList::default().add_as_name(&wd.id),
            Statement::Module(m) => NameList::default().add_as_name(&m.id),
            Statement::Function(f) => NameList::default().add_as_name(&f.id),
            Statement::InnerAttribute(_) => NameList::default(),

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
    fn names(&self) -> NameList {
        self.plan
            .names()
            .merge(self.body.names())
            .add_as_name(&self.id)
            .drop_locals()
    }
}

impl Names for ParameterList {
    fn names(&self) -> NameList {
        NameList::from_iter(self.iter().map(|param| &param.id)).merge_many(
            self.iter()
                .filter_map(|param| param.default_value.as_ref())
                .map(|expr| expr.names()),
        )
    }
}

impl Names for FunctionDefinition {
    fn names(&self) -> NameList {
        self.signature
            .names()
            .merge(self.body.names())
            .drop_locals()
    }
}

impl Names for FunctionSignature {
    fn names(&self) -> NameList {
        self.parameters.names()
    }
}

impl Names for InitDefinition {
    fn names(&self) -> NameList {
        self.parameters
            .names()
            .merge(self.body.names())
            .drop_locals()
    }
}

impl Names for ReturnStatement {
    fn names(&self) -> NameList {
        if let Some(result) = &self.result {
            result.names()
        } else {
            NameList::default()
        }
    }
}

impl Names for IfStatement {
    fn names(&self) -> NameList {
        todo!()
    }
}

impl Names for AssignmentStatement {
    fn names(&self) -> NameList {
        let names = self.assignment.expression.names();
        if matches!(self.assignment.qualifier, Qualifier::Const) {
            names.add_as_name(&self.assignment.id)
        } else {
            names.add_local(&self.assignment.id)
        }
    }
}

impl Names for ExpressionStatement {
    fn names(&self) -> NameList {
        self.expression.names()
    }
}

impl Names for Expression {
    fn names(&self) -> NameList {
        match self {
            Expression::Invalid
            | Expression::Value(_)
            | Expression::Literal(_)
            | Expression::Marker(_) => NameList::default(),

            Expression::FormatString(fs) => fs.names(),
            Expression::ArrayExpression(ae) => ae.names(),
            Expression::TupleExpression(te) => te.names(),
            Expression::Body(body) => body.names(),
            Expression::Call(call) => call.names(),
            Expression::QualifiedName(name) => name.into(),
            Expression::BinaryOp {
                lhs, op: _, rhs, ..
            } => lhs.names().merge(rhs.names()),
            Expression::UnaryOp { op: _, rhs, .. } => rhs.names(),
            Expression::ArrayElementAccess(e, e1, ..) => e.names().merge(e1.names()),
            Expression::PropertyAccess(e, ..) => e.names(),
            Expression::AttributeAccess(e, ..) => e.names(),
            Expression::MethodCall(e, mc, ..) => e.names().merge(mc.names()),
        }
    }
}

impl Names for Body {
    fn names(&self) -> NameList {
        self.statements.names()
    }
}

impl Names for UseStatement {
    fn names(&self) -> NameList {
        match &self.decl {
            UseDeclaration::Use(name) | UseDeclaration::UseAll(name) => name.into(),
            UseDeclaration::UseAlias(name, id) => {
                NameList::default().add_name(name).add_as_name(id)
            }
        }
    }
}

impl Names for FormatString {
    fn names(&self) -> NameList {
        NameList::default().merge_many(self.0.iter().filter_map(|inner| {
            if let FormatStringInner::FormatExpression(expr) = inner {
                Some(expr.expression.names())
            } else {
                None
            }
        }))
    }
}

impl Names for ArrayExpression {
    fn names(&self) -> NameList {
        self.inner.names()
    }
}

impl Names for ArrayExpressionInner {
    fn names(&self) -> NameList {
        match self {
            ArrayExpressionInner::List(expressions) => {
                NameList::default().merge_many(expressions.iter().map(|expr| expr.names()))
            }
            ArrayExpressionInner::Range(range_expression) => range_expression
                .first
                .names()
                .merge(range_expression.last.names()),
        }
    }
}

impl Names for RangeFirst {
    fn names(&self) -> NameList {
        self.0.names()
    }
}

impl Names for RangeLast {
    fn names(&self) -> NameList {
        self.0.names()
    }
}

impl Names for TupleExpression {
    fn names(&self) -> NameList {
        self.args.names()
    }
}

impl Names for Call {
    fn names(&self) -> NameList {
        self.argument_list.names().add_name(&self.name)
    }
}

impl Names for ArgumentList {
    fn names(&self) -> NameList {
        NameList::default().merge_many(
            self.iter()
                // get expressions out of arguments
                .map(|arg| arg.value.as_ref())
                .map(|expr| expr.names()),
        )
    }
}

impl Names for MethodCall {
    fn names(&self) -> NameList {
        self.argument_list.names().add_name(&self.name)
    }
}
