use pest::{Parser, iterators::Pair};
use thiserror::Error;

use crate::algebra::{
    CypherPrint, CypherScan, MQLPrint, MQLScan, SQLPrint, SQLScan, StorePrint, StoreScan, Window,
};
use crate::expression::{BinExpOp, UnExpOp};
use crate::folder::{AlgebraFolder, ExpressionFolder};
use crate::{
    algebra::{Algebra, Filter, Project, UnaryAlgebra},
    expression::Expression,
    parser::{LangParser, Rule},
};

use hyperion_value::{Float, Integer, Value};

pub trait FromPair<'i>: Sized {
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self, ASTError>;
}

#[derive(Debug, Error)]
pub enum ASTError {
    #[error("Parsing error: {0}")]
    ParsingError(String),
    #[error("Missing rule: {0:?}")]
    MissingRule(Rule),
    #[error("Unexpected rule {0:?}")]
    UnexpectedRule(Rule),
}

#[derive(Debug, Clone, Copy)]
pub struct Ref(usize);

#[derive(Debug)]
pub struct LogicalExpression {
    root: Ref,
    expressions: Vec<Expression>,
}

impl LogicalExpression {
    pub fn parse(expr: &str) -> Result<Self, ASTError> {
        let pairs = LangParser::parse(Rule::expr, expr)
            .map_err(|e| ASTError::ParsingError(format!("Expression parsing error {e}")))?;

        let pair = pairs.into_iter().next().unwrap();

        Self::parse_pair(pair)
    }

    pub fn parse_pair(pair: Pair<'_, Rule>) -> Result<Self, ASTError> {
        let mut expressions = Vec::new();

        let expr = pair.into_inner().next().unwrap();
        let root = Self::build_expression(expr, &mut expressions)?;

        Ok(Self { root, expressions })
    }

    fn add_expression(expr: Expression, expressions: &mut Vec<Expression>) -> Ref {
        let index = expressions.len();
        expressions.push(expr);
        Ref(index)
    }

    fn build_expression(
        expr: Pair<'_, Rule>,
        expressions: &mut Vec<Expression>,
    ) -> Result<Ref, ASTError> {
        let expr_ref = match expr.as_rule() {
            Rule::expr => {
                let mut inner = expr.into_inner();
                Self::build_expression(inner.next().unwrap(), expressions)?
            }

            Rule::escaper => {
                let mut inner = expr.into_inner();
                let _identifier = inner.next().unwrap();
                let content = inner.next().unwrap().as_str();
                let escape = Expression::Escape(content.to_owned());
                Self::add_expression(escape, expressions)
            }
            Rule::unary_math_functions => {
                let mut inner = expr.into_inner();

                let callable = inner.next().unwrap();
                let op = UnExpOp::from_pair(callable)?;
                let content = inner.next().unwrap();
                let content = Self::build_expression(content, expressions)?;

                let unary = Expression::UnaryExpression { op, value: content };
                Self::add_expression(unary, expressions)
            }
            Rule::TARGET => {
                let target = Expression::Target(expr.as_str().to_string());
                Self::add_expression(target, expressions)
            }
            // Primitive types
            Rule::FLOAT => {
                let literal = Expression::Literal(Value::Float(Float::from(
                    expr.as_str().parse::<f64>().unwrap(),
                )));
                Self::add_expression(literal, expressions)
            }
            Rule::INTEGER => {
                let literal = Expression::Literal(Value::Integer(Integer::from(
                    expr.as_str().parse::<i64>().unwrap(),
                )));
                Self::add_expression(literal, expressions)
            }
            Rule::BOOL => {
                let literal = match expr.as_str() {
                    "TRUE" => Expression::Literal(Value::Boolean(true)),
                    "FALSE" => Expression::Literal(Value::Boolean(false)),
                    _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
                };

                Self::add_expression(literal, expressions)
            }
            Rule::TEXT => {
                let literal = Expression::Literal(Value::String(expr.as_str().to_string()));
                Self::add_expression(literal, expressions)
            }
            Rule::VALUE => {
                let value = Value::from_pair(expr)?;
                let literal = Expression::Literal(value);
                Self::add_expression(literal, expressions)
            }
            _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
        };

        Ok(expr_ref)
    }
}

#[derive(Debug)]
pub struct LogicalPlan {
    root: Ref,
    algebra: Vec<Algebra>,
}

impl LogicalPlan {
    pub fn parse(query: &str) -> Result<Self, ASTError> {
        let pairs = LangParser::parse(Rule::query, query)
            .map_err(|e| ASTError::ParsingError(format!("Algebra parsing error {e}")))?;

        let query = pairs.into_iter().next().unwrap();
        let algebra_query = query
            .into_inner()
            .into_iter()
            .find(|p| p.as_rule() == Rule::algebra)
            .ok_or(ASTError::MissingRule(Rule::algebra))?;

        let mut algebra = Vec::new();

        Self::build_ast(algebra_query, &mut algebra)?;

        Ok(LogicalPlan {
            root: Ref(0),
            algebra,
        })
    }

    fn build_ast(query: Pair<'_, Rule>, algebra: &mut Vec<Algebra>) -> Result<(), ASTError> {
        let mut current = Ref(1);
        for operation in query.into_inner().rev() {
            current = Self::build_algebra(operation, current, algebra)?;
        }
        Ok(())
    }

    fn build_algebra(
        pair: Pair<'_, Rule>,
        input: Ref,
        algebra: &mut Vec<Algebra>,
    ) -> Result<Ref, ASTError> {
        let alg = match pair.as_rule() {
            Rule::into => {
                let mut inner = pair.into_inner();

                let identifier = inner.next().unwrap();
                let target = inner.next().unwrap().as_str().to_string();

                let print = match identifier.as_rule() {
                    Rule::SQL_IDENTIFIER => StorePrint::SQLPrint(SQLPrint { table: target }),
                    Rule::MQL_IDENTIFIER => StorePrint::MQLPrint(MQLPrint { collection: target }),
                    Rule::CYPHER_IDENTIFIER => {
                        StorePrint::CypherPrint(CypherPrint { graph: target })
                    }
                    _ => {
                        return Err(ASTError::UnexpectedRule(identifier.as_rule()));
                    }
                };

                Algebra::Print { print, input }
            }
            Rule::from => {
                let mut inner = pair.into_inner();
                let identifier = inner.next().unwrap();
                let target = inner.next().unwrap().as_str().to_string();

                let scan = match identifier.as_rule() {
                    Rule::SQL_IDENTIFIER => StoreScan::SQLScan(SQLScan { table: target }),
                    Rule::MQL_IDENTIFIER => StoreScan::MQLScan(MQLScan { collection: target }),
                    Rule::CYPHER_IDENTIFIER => StoreScan::CypherScan(CypherScan { graph: target }),
                    _ => {
                        return Err(ASTError::UnexpectedRule(identifier.as_rule()));
                    }
                };

                Algebra::Scan(scan)
            }
            Rule::filter => {
                let mut inner = pair.into_inner();
                let expr_ast = LogicalExpression::parse_pair(inner.next().unwrap())?;

                Algebra::Unary {
                    alg: UnaryAlgebra::Filter(Filter {
                        expression: expr_ast,
                    }),
                    input,
                }
            }
            Rule::project => {
                let mut expressions = Vec::new();
                for inner in pair.into_inner().into_iter() {
                    let expr_ast = LogicalExpression::parse_pair(inner)?;
                    expressions.push(expr_ast);
                }

                Algebra::Unary {
                    alg: UnaryAlgebra::Project(Project {
                        expression: expressions,
                    }),
                    input: input,
                }
            }
            Rule::count_window => {
                let mut inner = pair.into_inner();
                let size = inner.next().unwrap().as_str().replace("size=", "");
                let size = Integer::from(size.parse::<i64>().unwrap());
                Algebra::Unary {
                    alg: UnaryAlgebra::Window(Window::CountWindow { column: None, size }),
                    input,
                }
            }
            Rule::tumble_window => {
                let mut inner = pair.into_inner();
                let column = inner.next().unwrap().as_str().to_string();
                let size = inner.next().unwrap().as_str().replace("size=", "");
                let size = Integer::from(size.parse::<i64>().unwrap());
                Algebra::Unary {
                    alg: UnaryAlgebra::Window(Window::TumbleWindow {
                        column: Some(column),
                        size,
                    }),
                    input,
                }
            }
            _ => return Err(ASTError::UnexpectedRule(pair.as_rule())),
        };

        algebra.push(alg);
        let index = algebra.len() + 1;

        Ok(Ref(index))
    }
}

impl<'i> FromPair<'i> for Value {
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self, ASTError> {
        match pair.as_rule() {
            Rule::FLOAT => Ok(Value::Float(Float::from(
                pair.as_str().parse::<f64>().unwrap(),
            ))),
            Rule::INTEGER => Ok(Value::Integer(Integer::from(
                pair.as_str().parse::<i64>().unwrap(),
            ))),
            Rule::BOOL => match pair.as_str() {
                "TRUE" => Ok(Value::Boolean(true)),
                "FALSE" => Ok(Value::Boolean(false)),
                _ => Err(ASTError::UnexpectedRule(pair.as_rule())),
            },
            Rule::TEXT => Ok(Value::String(pair.as_str().to_string())),
            _ => Err(ASTError::UnexpectedRule(pair.as_rule())),
        }
    }
}

pub trait AlgebraVisitor {
    type Output;
    type Error: From<ASTError>;

    fn visit_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error>;
    fn visit_print(&mut self, print: &StorePrint) -> Result<Self::Output, Self::Error>;
    fn visit_project(&mut self, project: &Project) -> Result<Self::Output, Self::Error>;
    fn visit_filter(&mut self, filter: &Filter) -> Result<Self::Output, Self::Error>;
}

impl LogicalExpression {
    pub fn fold<E: ExpressionFolder>(&self, folder: &mut E) -> Result<E::Output, E::Error> {
        self.fold_expression(self.root, folder)
    }

    fn fold_expression<E: ExpressionFolder>(
        &self,
        index: Ref,
        folder: &mut E,
    ) -> Result<E::Output, E::Error> {
        if let Some(expr) = self.expressions.get(index.0) {
            match expr {
                Expression::Literal(literal) => folder.fold_literal(literal),
                Expression::Escape(escape) => folder.fold_escape(escape),
                Expression::Target(target) => folder.fold_target(target),
                Expression::UnaryExpression { op, value } => {
                    let inner = self.fold_expression(*value, folder)?;
                    folder.fold_unary_expression(op, inner)
                }
                Expression::BinaryExpression { op, left, right } => {
                    let inner_left = self.fold_expression(*left, folder)?;
                    let inner_right = self.fold_expression(*right, folder)?;
                    folder.fold_binary_expression(op, inner_left, inner_right)
                }
            }
        } else {
            Err(ASTError::ParsingError("Index doesn't exist".to_string()).into())
        }
    }
}

impl LogicalPlan {
    pub fn fold<F: AlgebraFolder>(&self, folder: &mut F) -> Result<F::Output, F::Error> {
        self.fold_algebra(self.root, folder)
    }

    fn fold_algebra<F: AlgebraFolder>(
        &self,
        index: Ref,
        folder: &mut F,
    ) -> Result<F::Output, F::Error> {
        if let Some(alg) = self.algebra.get(index.0) {
            match alg {
                // Root node
                Algebra::Print { print, input } => {
                    let inner = self.fold_algebra(*input, folder)?;
                    folder.fold_print(print, inner)
                }
                // Tree node
                Algebra::Unary { alg, input } => {
                    let inner = self.fold_algebra(*input, folder)?;
                    match alg {
                        UnaryAlgebra::Filter(filter) => folder.fold_filter(filter, inner),
                        UnaryAlgebra::Project(project) => folder.fold_project(project, inner),
                        UnaryAlgebra::Window(window) => folder.fold_window(window, inner),
                    }
                }
                // Tree node
                Algebra::Binary { alg, left, right } => {
                    let inner_left = self.fold_algebra(*left, folder)?;
                    let inner_right = self.fold_algebra(*right, folder)?;
                    match alg {
                        _ => todo!(),
                    }
                }
                // Leaf node
                Algebra::Scan(scan) => folder.fold_scan(scan),
            }
        } else {
            Err(ASTError::ParsingError("Index doesn't exist".to_string()).into())
        }
    }
}
