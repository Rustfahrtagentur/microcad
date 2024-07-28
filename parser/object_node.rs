lazy_static::lazy_static! {
    static ref PRATT_PARSER: PrattParser<Rule> = {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(object_node_union, Left) | Op::infix(object_node_difference, Left))
            .op(Op::infix(object_node_intersection, Left) | Op::infix(object_node_xor, Left))
    };
}

#[derive(Default)]
pub enum ObjectNodeExpression {
    /// Something went wrong (and an error will be reported)
    #[default]
    Invalid,

    /// A string literal. The .0 is the content of the string, without the quotes
    ObjectNodeNested(Vec<Call>),

    QualifiedName(QualifiedName),

    /// A binary operation: a | b
    BinaryOp {
        lhs: Box<ObjectNodeExpression>,
        /// '|', '-', '&', '^'
        op: char,
        rhs: Box<ObjectNodeExpression>,
    },

    /// A unary operation: !a
    UnaryOp {
        /// '!'
        op: char,
        rhs: Box<ObjectNodeExpression>,
    },
}
