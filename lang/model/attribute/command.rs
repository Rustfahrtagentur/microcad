// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{eval::AttributeError, model::Model, syntax::Identifier, value::Value};

#[derive(thiserror::Error, Debug)]
pub enum CommandError {}

pub trait Command {
    fn id(&self) -> Identifier;

    fn execute(&self, model: &Model) -> Result<Value, CommandError>;
}

pub trait NameValue {
    fn id(&self) -> Identifier;
    fn get_value(&self, model: &Model) -> Result<Value, AttributeError>;
    fn set_value(&mut self, model: &Model);
}
