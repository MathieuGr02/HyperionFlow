use std::fmt::Display;

use hyperion_value::Value;
use pest::iterators::Pair;

use crate::{
    arena_tree::Ref,
    ast::{ASTError, FromPair},
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

impl Expression {
    pub fn get_type(&self) -> ExpressionType {
        match self {
            Expression::Escape(_) => ExpressionType::Escape,
            Expression::Literal(value) => ExpressionType::Literal,
            Expression::Target(_) => ExpressionType::Target,
            Expression::UnaryExpression { op, value } => ExpressionType::UnaryExpression,
            Expression::BinaryExpression { op, left, right } => ExpressionType::BinaryExpression,
        }
    }
}

pub enum ExpressionType {
    Escape,
    Literal,
    Target,
    UnaryExpression,
    BinaryExpression,
}

impl Display for ExpressionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            ExpressionType::Escape => "ESCAPE",
            ExpressionType::Literal => "LITERAL",
            ExpressionType::Target => "TARGET",
            ExpressionType::UnaryExpression => "UNARY",
            ExpressionType::BinaryExpression => "BINARY",
        };

        write!(f, "{s}")
    }
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

impl Display for BinExpOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            BinExpOp::Rename => "RENAME",
            BinExpOp::Eq => "EQ",
            BinExpOp::Geq => "GEQ",
            BinExpOp::Leq => "LEQ",
            BinExpOp::Between => "BETWEEN",
        };

        write!(f, "{s}")
    }
}
