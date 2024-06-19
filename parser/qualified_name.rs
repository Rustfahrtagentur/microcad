// Resolve a qualified name to a type or value.

use crate::QualifiedName;

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_qualified_name() {
        let used_modules = vec!["shape2d", "math"];

        let qualified_names: Vec<QualifiedName> = vec!["shape2d.circle".into(), "math.PI".into()];
    }
}
