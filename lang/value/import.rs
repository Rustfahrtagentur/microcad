// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value importer

use std::rc::Rc;

use crate::{
    Id,
    diag::PushDiag,
    eval::{ArgumentMap, ArgumentMatch, ParameterValueList},
    parameter,
    resolve::Symbol,
    syntax::Identifier,
    value::Value,
};

use microcad_core::Integer;
use thiserror::Error;

/// Export error stub.
#[derive(Error, Debug)]
pub enum ImportError {
    /// IO Error.
    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    #[error("No importer found for file extension `{0}`")]
    NoImporterForFileExtension(Id),

    #[error("No importer found with id `{0}`")]
    NoImporterWithId(Id),

    #[error("Multiple exporters for file extension: {0:?}")]
    MultipleImportersForFileExtension(Vec<Id>),
}

pub trait Importer {
    fn parameters(&self) -> ParameterValueList;

    /// Import a value with parameters as argument map.
    fn import(&self, args: &ArgumentMap) -> Result<Value, ImportError>;

    fn id(&self) -> Id;

    /// Return file extensions
    fn file_extensions(&self) -> Vec<Id>;
}

#[derive(Default)]
pub struct ImporterRegistry {
    by_id: std::collections::HashMap<Id, std::rc::Rc<dyn Importer>>,
    by_file_extension: std::collections::HashMap<Id, Vec<std::rc::Rc<dyn Importer>>>,
    cache: std::collections::HashMap<(String, String), Value>,
}

impl ImporterRegistry {
    fn add(&mut self, importer: impl Importer + 'static) -> Result<(), ImportError> {
        let rc = Rc::new(importer);
        let id = rc.id();
        assert!(!id.is_empty());

        if self.by_id.contains_key(&id) {
            panic!("Importer already exists");
        }

        self.by_id.insert(id, rc.clone());

        let extensions = rc.file_extensions();
        for ext in extensions {
            if !ext.is_empty() && self.by_file_extension.contains_key(&ext) {
                self.by_file_extension
                    .get_mut(&ext)
                    .expect("Exporter list")
                    .push(rc.clone());
            } else {
                self.by_file_extension.insert(ext, vec![rc.clone()]);
            }
        }

        Ok(())
    }

    pub fn by_id(&self, id: &Id) -> Result<&Rc<dyn Importer>, ImportError> {
        self.by_id
            .get(id)
            .ok_or(ImportError::NoImporterWithId(id.clone()))
    }

    pub fn by_filename(
        &self,
        filename: impl AsRef<std::path::Path>,
    ) -> Result<&Rc<dyn Importer>, ImportError> {
        let ext: Id = filename
            .as_ref()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .into();

        if let Some(importers) = self.by_file_extension.get(&ext) {
            match importers.len() {
                0 => {}
                1 => return Ok(importers.first().expect("One importer")),
                _ => {
                    return Err(ImportError::MultipleImportersForFileExtension(
                        importers.iter().map(|importer| importer.id()).collect(),
                    ));
                }
            }
        }

        Err(ImportError::NoImporterForFileExtension(ext.clone()))
    }

    pub(crate) fn get_cached(&self, filename: String, id: String) -> Option<Value> {
        self.cache.get(&(filename, id)).cloned()
    }

    pub(crate) fn cache(&mut self, filename: String, id: String, value: Value) {
        self.cache.insert((filename, id), value);
    }
}

struct DummyImporter;

impl Importer for DummyImporter {
    fn parameters(&self) -> ParameterValueList {
        vec![parameter!(some_arg: Integer = 32)].into()
    }

    fn import(&self, args: &ArgumentMap) -> Result<Value, ImportError> {
        let some_arg: Integer = args.get::<Integer>("some_arg");
        if some_arg == 32 {
            Ok(Value::Integer(32))
        } else {
            Ok(Value::Integer(42))
        }
    }

    fn id(&self) -> Id {
        Id::new("dummy")
    }

    fn file_extensions(&self) -> Vec<Id> {
        vec![Id::new("dummy"), Id::new("dmy")]
    }
}

fn import() -> Symbol {
    Symbol::new_builtin(
        Identifier::no_ref("import"),
        Some(
            vec![
                parameter!(filename: String),
                parameter!(id: String = String::new()),
            ]
            .into(),
        ),
        &|parameter_values, argument_values, context| match ArgumentMap::find_match(
            argument_values,
            parameter_values.expect("Parameter values"),
        ) {
            Ok(arg_map) => context.import(&arg_map),
            Err(err) => {
                context.error(argument_values, err)?;
                Ok(Value::None)
            }
        },
    )
}

#[test]
fn importer() {
    let mut registry = ImporterRegistry::default();

    registry.add(DummyImporter);

    let by_id = registry.by_id(&"dummy".into()).expect("Dummy importer");

    let mut args = ArgumentMap::new(crate::src_ref::SrcRef(None));
    args.insert(Identifier::no_ref("some_arg".into()), Value::Integer(32));

    let value = by_id.import(&args).expect("Value");
    assert!(matches!(value, Value::Integer(32)));

    let by_filename = registry.by_filename("test.dmy").expect("Filename");

    args.insert(Identifier::no_ref("some_arg".into()), Value::Integer(42));
    let value = by_id.import(&args).expect("Value");

    assert!(matches!(value, Value::Integer(42)));
}
