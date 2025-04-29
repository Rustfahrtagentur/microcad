// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module statement syntax elements

use crate::{rc::*, src_ref::*, syntax::*};

mod assignment;
mod if_statement;
mod marker_statement;
mod return_statement;

pub use assignment::*;
pub use if_statement::*;
pub use marker_statement::*;
pub use return_statement::*;

/// Module statement
#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum Statement {
    /// Module definition
    Module(Rc<ModuleDefinition>),
    /// Namespace definition
    Namespace(Rc<NamespaceDefinition>),
    /// Function definition
    Function(Rc<FunctionDefinition>),
    /// Module init definition
    ModuleInit(Rc<ModuleInitDefinition>),

    /// Use statement
    Use(UseStatement),
    /// Return statement
    Return(ReturnStatement),
    /// If statement
    If(IfStatement),
    /// Marker statement
    Marker(Marker),

    /// Assignment
    Assignment(Assignment),
    /// Expression
    Expression(Expression),
}

impl SrcReferrer for Statement {
    fn src_ref(&self) -> SrcRef {
        match self {
            Self::Module(md) => md.src_ref(),
            Self::Namespace(ns) => ns.src_ref(),
            Self::Function(fd) => fd.src_ref(),
            Self::ModuleInit(mid) => mid.src_ref(),

            Self::Use(us) => us.src_ref(),
            Self::Return(r) => r.src_ref(),
            Self::If(i) => i.src_ref(),
            Self::Marker(m) => m.src_ref(),

            Self::Assignment(a) => a.src_ref(),
            Self::Expression(e) => e.src_ref(),
        }
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Module(m) => {
                write!(f, "{m}")
            }
            Self::Namespace(ns) => {
                write!(f, "{}", ns.id)
            }
            Self::Function(_f) => {
                write!(f, "{}", _f.id)
            }
            Self::ModuleInit(mi) => {
                write!(f, "{mi}")
            }

            Self::Use(u) => write!(f, "{u};"),
            Self::Return(r) => write!(f, "{r};"),
            Self::If(i) => write!(f, "{i}"),
            Self::Marker(m) => write!(f, "{m};"),

            Self::Assignment(a) => write!(f, "{a};"),
            Self::Expression(e) => write!(f, "{e};"),
        }
    }
}

impl PrintSyntax for Statement {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$}Statement:", "")?;
        match self {
            Self::Module(m) => m.print_syntax(f, depth + 1),
            Self::Namespace(ns) => ns.print_syntax(f, depth + 1),
            Self::Function(func) => func.print_syntax(f, depth + 1),
            Self::ModuleInit(mi) => mi.print_syntax(f, depth + 1),

            Self::Use(u) => u.print_syntax(f, depth + 1),
            Self::Return(r) => r.print_syntax(f, depth + 1),
            Self::If(i) => i.print_syntax(f, depth + 1),
            Self::Marker(m) => m.print_syntax(f, depth + 1),

            Self::Assignment(a) => a.print_syntax(f, depth + 1),
            Self::Expression(e) => e.print_syntax(f, depth + 1),
        }
    }
}
