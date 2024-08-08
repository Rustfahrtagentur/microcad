use crate::identifier::{Identifier, QualifiedName};
use crate::module::UseStatement;
use crate::parser::*;

use core::fmt;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SyntaxError(Box<pest::error::Error<Rule>>),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(value)
    }
}

#[derive(Default)]
pub struct Document {
    path: Option<std::path::PathBuf>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_path(path: impl AsRef<std::path::Path>) -> Self {
        Self {
            path: Some(std::path::PathBuf::from(path.as_ref())),
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.path)
    }
}

pub trait Depth {
    fn depth(self) -> usize;
}
