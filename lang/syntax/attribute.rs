// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// A subcommand for an [`AttributeCommand`].
#[derive(Debug, Clone)]
pub enum AttributeCommand {
    /// A command with an optional identifier and optional arguments: `width(offset = 30mm)`.
    Call(Option<Identifier>, Option<ArgumentList>),
    /// A format string subcommand: `"test.svg"`.
    Expression(Expression),
}

impl std::fmt::Display for AttributeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttributeCommand::Call(id, argument_list) => {
                write!(
                    f,
                    "{id}{argument_list}",
                    id = match id {
                        Some(id) => format!("{id}"),
                        None => String::new(),
                    },
                    argument_list = match argument_list {
                        Some(argument_list) => format!("({argument_list})"),
                        None => String::new(),
                    }
                )
            }
            AttributeCommand::Expression(expression) => write!(f, "{expression}"),
        }
    }
}

impl SrcReferrer for AttributeCommand {
    fn src_ref(&self) -> SrcRef {
        match &self {
            AttributeCommand::Call(identifier, argument_list) => {
                match (identifier, argument_list) {
                    (None, None) => unreachable!("Invalid AttributeCommand::Call"),
                    (None, Some(arguments)) => arguments.src_ref(),
                    (Some(identifier), None) => identifier.src_ref(),
                    (Some(identifier), Some(arguments)) => SrcRef::merge(identifier, arguments),
                }
            }
            AttributeCommand::Expression(expression) => expression.src_ref(),
        }
    }
}

/// An attribute item.
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The id of the attribute.
    pub id: Identifier,
    /// Attribute commands: `width, height(30mm)`.
    pub commands: Vec<AttributeCommand>,
    /// Tells if the attribute is an inner attribute: `#[...]` (outer) vs `#![...]` (inner).
    pub is_inner: bool,
    /// Source reference
    pub src_ref: SrcRef,
}

impl Attribute {
    /// Return some command it is the only one in the list.
    pub fn single_command(&self) -> Option<&AttributeCommand> {
        match self.commands.len() {
            1 => self.commands.first(),
            _ => None,
        }
    }
}

impl PrintSyntax for Attribute {
    fn print_syntax(&self, f: &mut std::fmt::Formatter, depth: usize) -> std::fmt::Result {
        writeln!(f, "{:depth$} Attribute: {}", "", self.to_string())
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.is_inner {
            true => write!(f, "#![")?,
            false => write!(f, "#[")?,
        }
        match self.commands.is_empty() {
            true => write!(f, "{}", self.id)?,
            false => write!(
                f,
                "{} = {}",
                self.id,
                self.commands
                    .iter()
                    .map(|command| command.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?,
        }
        writeln!(f, "]")
    }
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        self.src_ref.clone()
    }
}

/// A list of attributes, e.g. `#foo #[bar, baz = 42]`
#[derive(Debug, Clone, Default, Deref, DerefMut)]
pub struct AttributeList(Vec<Attribute>);

impl std::fmt::Display for AttributeList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_for_each(|attr| writeln!(f, "{attr}"))
    }
}

impl SrcReferrer for AttributeList {
    fn src_ref(&self) -> SrcRef {
        if self.0.is_empty() {
            SrcRef(None)
        } else {
            SrcRef::merge(
                &self.0.first().expect("One element").src_ref(),
                &self.0.last().expect("Second element").src_ref(),
            )
        }
    }
}
