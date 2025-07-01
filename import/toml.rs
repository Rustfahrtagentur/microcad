// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Import values from TOML

use microcad_lang::{Id, builtin::*, syntax::*, value::*};

/// Import TOML files into a tuple.
pub struct TomlImporter;

impl TomlImporter {
    fn toml_to_value(toml: &toml::Value) -> Value {
        match toml {
            toml::Value::String(s) => Value::String(s.clone()),
            toml::Value::Integer(i) => Value::Integer(*i),
            toml::Value::Float(f) => (*f).into(),
            toml::Value::Boolean(b) => Value::Bool(*b),
            toml::Value::Datetime(_) => todo!(),
            toml::Value::Array(values) => {
                let mut list = Vec::new();
                for toml_value in values {
                    list.push(Self::toml_to_value(toml_value));
                }
                Value::Array(Array::new(
                    ValueList::new(list),
                    microcad_lang::ty::Type::Invalid, // TODO get common type here.
                ))
            }
            toml::Value::Table(map) => Value::Tuple(Box::new(Tuple::new_named(
                map.iter()
                    .map(|(k, v)| (Identifier::no_ref(k), Self::toml_to_value(v)))
                    .collect(),
            ))),
        }
    }
}

impl Importer for TomlImporter {
    fn import(
        &self,
        args: &microcad_lang::eval::ArgumentMap,
    ) -> Result<microcad_lang::value::Value, microcad_lang::builtin::ImportError> {
        let filename = args.get::<String>("filename");
        let content = std::fs::read_to_string(filename)?;

        Ok(Self::toml_to_value(
            &toml::from_str::<toml::Value>(&content)
                .map_err(|err| ImportError::CustomError(Box::new(err)))?,
        ))
    }
}

impl FileIoInterface for TomlImporter {
    fn id(&self) -> microcad_lang::Id {
        Id::new("toml")
    }
}

#[test]
fn toml_importer() {
    use microcad_lang::GetPropertyValue;

    // Import a toml from `Cargo.toml` and convert it into a tuple.
    let toml_importer = TomlImporter;

    let mut args = microcad_lang::eval::ArgumentMap::default();
    args.insert(
        Identifier::no_ref("filename"),
        Value::String("Cargo.toml".into()),
    );
    let value = toml_importer.import(&args).expect("No error");
    println!("{value}");

    if let Value::Tuple(tuple) = value {
        let package = tuple
            .by_id(&Identifier::no_ref("package"))
            .expect("Package info");
        let name = package.get_property_value(&Identifier::no_ref("name"));

        let name = name.try_string().expect("String value");
        println!("{name}");
    } else {
        panic!("Value must be a tuple!")
    }
}
