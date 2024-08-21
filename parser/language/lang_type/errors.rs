use crate::language::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TypeError {
    #[error("Invalid map key type: {0}")]
    InvalidMapKeyType(String),

    #[error("Duplicated field name in map: {0}")]
    DuplicatedMapField(Identifier),
}
