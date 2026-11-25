use hyperion_value::Value;
use pest::iterators::Pair;

use crate::{
    ast::{ASTError, FromPair, Ref},
    parser::Rule,
};

#[derive(Debug)]
pub enum Expression {
    // Leaf node expressions
    Escape(String),
    Literal(Value),
    Target(String),

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
pub enum UnExpOp {
    Abs,
    Avg,
    Ceiling,
    Count,
    Exp,
    Floor,
    Max,
    Min,
    Sign,
    Sqrt,
    Square,
    Sum,
}

impl<'i> FromPair<'i> for UnExpOp {
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self, ASTError> {
        match pair.as_str() {
            "ABS" => Ok(Self::Abs),
            "AVG" => Ok(Self::Avg),
            "CEILING" => Ok(Self::Ceiling),
            "COUNT" => Ok(Self::Count),
            "EXP" => Ok(Self::Exp),
            "FLOOR" => Ok(Self::Floor),
            "MAX" => Ok(Self::Max),
            "MIN" => Ok(Self::Min),
            "SIGN" => Ok(Self::Sign),
            "SQRT" => Ok(Self::Sqrt),
            "SQUARE" => Ok(Self::Square),
            "SUM" => Ok(Self::Sum),
            _ => Err(ASTError::ParsingError(format!(
                "Unary function {} doesn't exist",
                pair.as_str()
            ))),
        }
    }
}

#[derive(Debug)]
pub enum BinExpOp {
    Rename,
    // Boolean expressions
    Eq,
    Geq,
    Leq,
    Between,
}
