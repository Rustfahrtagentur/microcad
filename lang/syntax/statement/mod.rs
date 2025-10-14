// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! statement syntax elements

use crate::{rc::*, src_ref::*, syntax::*};

mod assignment_statement;
mod expression_statement;
mod if_statement;
mod return_statement;
mod statement_list;

pub use assignment_statement::*;
pub use expression_statement::*;
pub use if_statement::*;
pub use return_statement::*;
pub use statement_list::*;

/// Any statement.
#[derive(Clone, strum::IntoStaticStr)]
pub enum Statement {
    /// Part definition
    Workbench(Rc<WorkbenchDefinition>),
    /// Module definition
    Module(Rc<ModuleDefinition>),
    /// Function definition
    Function(Rc<FunctionDefinition>),
    /// Init definition
    Init(Rc<InitDefinition>),

    /// Use statement
    Use(UseStatement),
    /// Return statement
    Return(ReturnStatement),
    /// If statement
    If(IfStatement),
    /// Inner attribute statement: `#![size = std::A4]`.
    InnerAttribute(Attribute),

    /// Assignment statement.
    Assignment(AssignmentStatement),
    /// Expression statement.
    Expression(ExpressionStatement),
}

impl SrcReferrer for Statement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Workbench(w) => w.src_ref(),
            Self::Module(m) => m.src_ref(),
            Self::Function(fd) => fd.src_ref(),
            Self::Init(mid) => mid.src_ref(),

            Self::Use(us) => us.src_ref(),
            Self::Return(r) => r.src_ref(),
            Self::If(i) => i.src_ref(),
            Self::InnerAttribute(i) => i.src_ref(),

            Self::Assignment(a) => a.src_ref(),
            Self::Expression(e) => e.src_ref(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Workbench(w) => {
                write!(f, "{w}")
            }
            Self::Module(m) => {
                write!(f, "{}", m.id)
            }
            Self::Function(_f) => {
                write!(f, "{}", _f.id)
            }
            Self::Init(mi) => {
                write!(f, "{mi}")
            }

            Self::Use(u) => write!(f, "{u};"),
            Self::Return(r) => write!(f, "{r};"),
            Self::If(i) => write!(f, "{i}"),
            Self::InnerAttribute(i) => write!(f, "{i}"),

            Self::Assignment(a) => write!(f, "{a}"),
            Self::Expression(e) => write!(f, "{e}"),
        }
    }
}
impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Workbench(w) => {
                write!(f, "{w:?}")
            }
            Self::Module(m) => {
                write!(f, "{:?}", m.id)
            }
            Self::Function(_f) => {
                write!(f, "{:?}", _f.id)
            }
            Self::Init(mi) => {
                write!(f, "{mi:?}")
            }

            Self::Use(u) => write!(f, "{u:?};"),
            Self::Return(r) => write!(f, "{r:?};"),
            Self::If(i) => write!(f, "{i:?}"),
            Self::InnerAttribute(i) => write!(f, "{i:?}"),

            Self::Assignment(a) => write!(f, "{a:?}"),
            Self::Expression(e) => write!(f, "{e:?}"),
        }
    }
}

impl TreeDisplay for Statement {
    fn tree_print(&self, f: &mut std::fmt::Formatter, depth: TreeState) -> std::fmt::Result {
        // statement is transparent
        match self {
            Self::Workbench(w) => w.tree_print(f, depth),
            Self::Module(m) => m.tree_print(f, depth),
            Self::Function(func) => func.tree_print(f, depth),
            Self::Init(i) => i.tree_print(f, depth),

            Self::Use(u) => u.tree_print(f, depth),
            Self::Return(r) => r.tree_print(f, depth),
            Self::If(i) => i.tree_print(f, depth),
            Self::InnerAttribute(i) => i.tree_print(f, depth),

            Self::Assignment(a) => a.tree_print(f, depth),
            Self::Expression(e) => e.tree_print(f, depth),
        }
    }
}
