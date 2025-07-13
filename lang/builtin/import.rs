// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Value importer

use std::rc::Rc;

use crate::{builtin::file_io::*, eval::*, syntax::*, value::*, Id};

use thiserror::Error;

/// Export error stub.
#[derive(Error, Debug)]
pub enum ImportError {
    /// IO Error.
    #[error("IO Error")]
    IoError(#[from] std::io::Error),

    /// The file was not found.
    #[error("File not found: {0}")]
    FileNotFound(std::path::PathBuf),

    /// No importer found for file.
    #[error("No importer found for file `{0}`")]
    NoImporterForFile(std::path::PathBuf),

    /// No importer with id.
    #[error("No importer found with id `{0}`")]
    NoImporterWithId(Id),

    /// Found multiple importers with same file extensions.
    #[error("Multiple importers for file extension: {0:?}")]
    MultipleImportersForFileExtension(Vec<Id>),

    /// Custom error.
    #[error("{0}")]
    CustomError(Box<dyn std::error::Error>),
}

/// An importer trait to import files of a specific type.
pub trait Importer: FileIoInterface {
    /// The parameters this importer accepts (empty by default).
    fn parameters(&self) -> ParameterValueList {
        ParameterValueList::default()
    }

    /// Import a value with parameters as argument map.
    fn import(&self, args: &Tuple) -> Result<Value, ImportError>;
}

/// Importer registry stores all importers.
#[derive(Default)]
pub struct ImporterRegistry {
    io: FileIoRegistry<Rc<dyn Importer + 'static>>,
    cache: std::collections::HashMap<(String, String), Value>,
}

impl ImporterRegistry {
    /// Add new importer to the registry.
    ///
    /// TODO Error handling.
    pub fn insert(mut self, importer: impl Importer + 'static) -> Self {
        let rc = Rc::new(importer);
        self.io.insert(rc);
        self
    }

    /// Get importer by id.
    pub fn by_id(&self, id: &Id) -> Result<Rc<dyn Importer>, ImportError> {
        self.io
            .by_id(id)
            .ok_or(ImportError::NoImporterWithId(id.clone()))
    }

    /// Get importer by filename.
    pub fn by_filename(
        &self,
        filename: impl AsRef<std::path::Path>,
    ) -> Result<Rc<dyn Importer>, ImportError> {
        let importers = self.io.by_filename(filename.as_ref());
        match importers.len() {
            0 => Err(ImportError::NoImporterForFile(std::path::PathBuf::from(
                filename.as_ref(),
            ))),
            1 => Ok(importers.first().expect("One importer").clone()),
            _ => Err(ImportError::MultipleImportersForFileExtension(
                importers.iter().map(|importer| importer.id()).collect(),
            )),
        }
    }

    pub(crate) fn get_cached(&self, filename: String, id: String) -> Option<Value> {
        self.cache.get(&(filename, id)).cloned()
    }

    pub(crate) fn cache(&mut self, filename: String, id: String, value: Value) {
        self.cache.insert((filename, id), value);
    }
}

/// Importer Registry Access.
pub trait ImporterRegistryAccess {
    /// Error type.
    type Error;

    /// Import a value from an argument map
    fn import(
        &mut self,
        args: &Tuple,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Value, Self::Error>;
}

impl ImporterRegistryAccess for ImporterRegistry {
    type Error = ImportError;

    fn import(
        &mut self,
        args: &Tuple,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Value, Self::Error> {
        let filename: String = args.get("filename");

        match [".".into()] // Search working dir first
            .iter()
            .chain(search_paths.iter())
            .map(|path| path.join(&filename))
            .find(|path| path.exists())
        {
            Some(path) => {
                let mut arg_map = args.clone();
                let filename = path.to_string_lossy().to_string();
                arg_map.insert(
                    Identifier::no_ref("filename"),
                    Value::String(filename.clone()),
                );
                let id: String = arg_map.get("id");

                // Check if value is in cache
                if let Some(value) = self.get_cached(filename.clone(), id.clone()) {
                    return Ok(value);
                }

                let value = if id.is_empty() {
                    self.by_filename(&filename)
                } else {
                    self.by_id(&id.clone().into())
                }?
                .import(&arg_map)?;
                self.cache(filename, id, value.clone());
                Ok(value)
            }
            None => Err(ImportError::FileNotFound(std::path::PathBuf::from(
                &filename,
            ))),
        }
    }
}

#[test]
fn importer() {
    struct DummyImporter;

    use crate::{builtin::Importer, parameter};
    use microcad_core::Integer;

    impl Importer for DummyImporter {
        fn parameters(&self) -> ParameterValueList {
            [parameter!(some_arg: Integer = 32)].into_iter().collect()
        }

        fn import(&self, args: &Tuple) -> Result<Value, ImportError> {
            let some_arg: Integer = args.get("some_arg");
            if some_arg == 32 {
                Ok(Value::Integer(32))
            } else {
                Ok(Value::Integer(42))
            }
        }
    }

    impl FileIoInterface for DummyImporter {
        fn id(&self) -> Id {
            Id::new("dummy")
        }

        fn file_extensions(&self) -> Vec<Id> {
            vec![Id::new("dummy"), Id::new("dmy")]
        }
    }

    let registry = ImporterRegistry::default().insert(DummyImporter);

    let by_id = registry.by_id(&"dummy".into()).expect("Dummy importer");

    let mut args = crate::tuple!("(some_arg=32)");

    let value = by_id.import(&args).expect("Value");
    assert!(matches!(value, Value::Integer(32)));

    let by_filename = registry.by_filename("test.dmy").expect("Filename");

    args.insert(Identifier::no_ref("some_arg"), Value::Integer(42));
    let value = by_filename.import(&args).expect("Value");

    assert!(matches!(value, Value::Integer(42)));
}
