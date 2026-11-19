use hyperion_value::Value;

use crate::ast::Ref;

#[derive(Debug)]
pub enum Expression {
    // Leaf node expressions
    Escape(String),
    Literal(Value),

    // Node unary expression
    // Represents the expression:
    // A
    // |
    // B
    UnaryExpression { op: UnExpOp, value: Ref },
    // Node binary expression
    // Represents the expression:
    //  A
    // / \
    // B  C
    BinaryExpression { op: BinExpOp, left: Ref, right: Ref },
}

#[derive(Debug)]
pub enum UnExpOp {}

#[derive(Debug)]
pub enum BinExpOp {
    Eq,
    Geq,
    Leq,
}
