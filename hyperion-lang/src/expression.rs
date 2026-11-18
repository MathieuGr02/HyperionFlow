use hyperion_value::Value;

pub(crate) type ExprRef = usize;

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
    UnaryExpression {
        op: UnExpOp,
        value: ExprRef,
    },
    // Node binary expression
    // Represents the expression:
    //  A
    // / \
    // B  C
    BinaryExpression {
        op: BinExpOp,
        left: ExprRef,
        right: ExprRef,
    },
}

#[derive(Debug)]
pub enum UnExpOp {}

#[derive(Debug)]
pub enum BinExpOp {
    Eq,
    Geq,
    Leq,
}
