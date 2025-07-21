// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Attribute syntax entities.

use crate::{src_ref::*, syntax::*};
use derive_more::{Deref, DerefMut};

/// A subcommand for an [`AttributeCommand`].
#[derive(Debug, Clone)]
pub enum AttributeSubcommand {
    /// A format string subcommand: `"test.svg"`.
    FormatString(FormatString),
    /// A command with an identifier and optional arguments: `width(offset = 30mm)`.
    Call(Identifier, Option<ArgumentList>),
}

impl std::fmt::Display for AttributeSubcommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            AttributeSubcommand::FormatString(format_string) => write!(f, "{format_string}"),
            AttributeSubcommand::Call(id, argument_list) => {
                write!(
                    f,
                    "{id}{argument_list}",
                    argument_list = match argument_list {
                        Some(argument_list) => format!("({argument_list})"),
                        None => String::new(),
                    }
                )
            }
        }
    }
}

impl SrcReferrer for AttributeSubcommand {
    fn src_ref(&self) -> SrcRef {
        match &self {
            AttributeSubcommand::FormatString(format_string) => format_string.src_ref(),
            AttributeSubcommand::Call(identifier, argument_list) => match argument_list {
                Some(argument_list) => {
                    SrcRef::merge(&identifier.src_ref(), &argument_list.src_ref())
                }
                None => identifier.src_ref(),
            },
        }
    }
}

/// An attribute command: `#[export: "test.svg"]`.
#[derive(Debug, Clone)]
pub struct AttributeCommand {
    /// Command id.
    pub id: Identifier,
    /// Subcommands.
    pub subcommands: Vec<AttributeSubcommand>,
    /// Source code reference.
    pub src_ref: SrcRef,
}

impl AttributeCommand {
    /// Return command id.
    pub fn id(&self) -> &Identifier {
        &self.id
    }
}

impl std::fmt::Display for AttributeCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{id}{subcommands}",
            id = self.id,
            subcommands = if self.subcommands.is_empty() {
                String::new()
            } else {
                format!(
                    ": {}",
                    self.subcommands
                        .iter()
                        .map(|subcommand| subcommand.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
    }
}

/// An attribute item.
#[derive(Debug, Clone)]
pub enum Attribute {
    /// An exporter attribute: `#[bar(baz = 32)]`.
    Exporter(Identifier, ArgumentList),
    /// A name value attribute: `#[color = "red"]`.
    NameValue(Identifier, Expression),
    /// A command attribute: `#[measure: width]`.
    Command(AttributeCommand),
}

impl Attribute {
    /// Return qualified name for this attribute.
    pub fn id(&self) -> &Identifier {
        match self {
            Attribute::Exporter(id, _) => id,
            Attribute::NameValue(id, _) => id,
            Attribute::Command(command) => command.id(),
        }
    }
}

impl std::fmt::Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Attribute::Exporter(id, argument_list) => {
                writeln!(f, "#[{id}({argument_list})]")
            }
            Attribute::NameValue(id, expression) => {
                writeln!(f, "#[{id} = {expression}]")
            }
            Attribute::Command(command) => {
                writeln!(f, "#[{command}]")
            }
        }
    }
}

impl SrcReferrer for Attribute {
    fn src_ref(&self) -> crate::src_ref::SrcRef {
        match self {
            Attribute::Exporter(id, argument_list) => SrcRef::merge(id, argument_list),
            Attribute::NameValue(id, expression) => SrcRef::merge(id, expression),
            Attribute::Command(command) => command.src_ref.clone(),
        }
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
