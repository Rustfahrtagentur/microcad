// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Module statement parser entities
//!
use crate::{parse::*, parser::*, src_ref::*};

/// Module statement
#[derive(Clone, Debug, strum::IntoStaticStr)]
pub enum Statement {
    /// Module definition
    Module(std::rc::Rc<ModuleDefinition>),
    /// Namespace definition
    Namespace(std::rc::Rc<NamespaceDefinition>),
    /// Function definition
    Function(std::rc::Rc<FunctionDefinition>),
    /// Module init definition
    ModuleInit(std::rc::Rc<ModuleInitDefinition>),

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

impl Parse for Statement {
    fn parse(pair: Pair) -> ParseResult<Self> {
        Parser::ensure_rule(&pair, Rule::statement);
        let first = pair.inner().next().expect(INTERNAL_PARSE_ERROR);
        Ok(match first.as_rule() {
            Rule::module_definition => Self::Module(std::rc::Rc::<ModuleDefinition>::parse(first)?),
            Rule::namespace_definition => {
                Self::Namespace(std::rc::Rc::<NamespaceDefinition>::parse(first)?)
            }
            Rule::function_definition => {
                Self::Function(std::rc::Rc::<FunctionDefinition>::parse(first)?)
            }
            Rule::module_init_definition => {
                Self::ModuleInit(std::rc::Rc::new(ModuleInitDefinition::parse(first)?))
            }

            Rule::use_statement => Self::Use(UseStatement::parse(first)?),
            Rule::return_statement => Self::Return(ReturnStatement::parse(first)?),
            Rule::if_statement => Self::If(IfStatement::parse(first)?),
            Rule::marker_statement => Self::Marker(Marker::parse(first)?),

            Rule::assignment => Self::Assignment(Assignment::parse(first)?),
            Rule::expression | Rule::expression_no_semicolon => {
                Self::Expression(Expression::parse(first)?)
            }
            rule => unreachable!(
                "Unexpected module statement, got {:?} {:?}",
                rule,
                first.clone()
            ),
        })
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Module(m) => {
                write!(f, "{m}")
            }
            Self::Namespace(ns) => {
                write!(f, "{}", ns.name)
            }
            Self::Function(_f) => {
                todo!()
                //write!(f, "{}", f.name)<
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
