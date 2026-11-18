use pest::{Parser, iterators::Pair};
use thiserror::Error;

use crate::algebra::{
    CypherPrint, CypherScan, MQLPrint, MQLScan, SQLPrint, SQLScan, StorePrint, StoreScan,
};
use crate::{
    algebra::{AlgRef, Algebra, Filter, Project, UnaryAlgebra},
    expression::{ExprRef, Expression},
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

#[derive(Debug)]
pub struct AST {
    root: AlgRef,
    algebra: Vec<Algebra>,
    expressions: Vec<Expression>,
}

impl AST {
    pub fn parse(query: &str) -> Result<Self, ASTError> {
        let pairs = LangParser::parse(Rule::query, query)
            .map_err(|e| ASTError::ParsingError(e.to_string()))?;

        let query = pairs.into_iter().next().unwrap();
        let algebra_query = query
            .into_inner()
            .into_iter()
            .find(|p| p.as_rule() == Rule::algebra)
            .ok_or(ASTError::MissingRule(Rule::algebra))?;

        dbg!(&algebra_query);

        let mut algebra = Vec::new();
        let mut expressions = Vec::new();

        Self::build_ast(algebra_query, &mut algebra, &mut expressions)?;

        Ok(AST {
            root: 0,
            algebra,
            expressions,
        })
    }

    fn build_ast(
        query: Pair<'_, Rule>,
        algebra: &mut Vec<Algebra>,
        expressions: &mut Vec<Expression>,
    ) -> Result<(), ASTError> {
        let mut current = 0;
        for operation in query.into_inner().rev() {
            current = Self::build_algebra(operation, current, algebra, expressions)?;
        }

        /*
                let mut operations = Vec::new();
                let mut scan = None;
                for inner in query.into_inner() {
                    match inner.as_rule() {
                        Rule::from => scan = Some(inner),
                        _ => operations.push(inner),
                    }
                }

                let scan = scan.ok_or(ASTError::MissingRule(Rule::from))?;
                let scan = Algebra::Scan(Self::build_scan(scan)?);

                let mut index = algebra.len();
                algebra.push(scan);

                for operation in operations.into_iter() {
                    index = Self::build_algebra(operation, index, algebra, expressions)?;
                }
        */
        Ok(())
    }

    fn build_algebra(
        pair: Pair<'_, Rule>,
        input: AlgRef,
        algebra: &mut Vec<Algebra>,
        expressions: &mut Vec<Expression>,
    ) -> Result<AlgRef, ASTError> {
        let alg = match pair.as_rule() {
            Rule::into => {
                let mut inner = pair.into_inner();

                let identifier = inner.next().unwrap();
                let target = inner.next().unwrap().as_str().to_string();

                let print = match identifier.as_rule() {
                    Rule::sql_identifier => StorePrint::SQLPrint(SQLPrint { table: target }),
                    Rule::mql_identifier => StorePrint::MQLPrint(MQLPrint { collection: target }),
                    Rule::cypher_identifier => {
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
                    Rule::sql_identifier => StoreScan::SQLScan(SQLScan { table: target }),
                    Rule::mql_identifier => StoreScan::MQLScan(MQLScan { collection: target }),
                    Rule::cypher_identifier => StoreScan::CypherScan(CypherScan { graph: target }),
                    _ => {
                        return Err(ASTError::UnexpectedRule(identifier.as_rule()));
                    }
                };

                Algebra::Scan(scan)
            }
            Rule::filter => {
                let mut inner = pair.into_inner();
                let expr_ref = Self::build_expression(inner.next().unwrap(), expressions)?;

                Algebra::Unary {
                    alg: UnaryAlgebra::Filter(Filter {
                        expression: expr_ref,
                    }),
                    input,
                }
            }
            Rule::project => {
                let mut inner = pair.into_inner();
                let expr_ref = Self::build_expression(inner.next().unwrap(), expressions)?;

                Algebra::Unary {
                    alg: UnaryAlgebra::Project(Project {
                        expression: expr_ref,
                    }),
                    input: input,
                }
            }
            _ => return Err(ASTError::UnexpectedRule(pair.as_rule())),
        };

        let index = algebra.len();
        algebra.push(alg);

        Ok(index)
    }

    fn build_expression(
        expr: Pair<'_, Rule>,
        expressions: &mut Vec<Expression>,
    ) -> Result<ExprRef, ASTError> {
        let expr = match expr.as_rule() {
            Rule::sql => {
                let sql_str = expr.as_str();
                Expression::Escape(sql_str.to_string())
            }
            Rule::mql => {
                let mql_str = expr.as_str();
                Expression::Escape(mql_str.to_string())
            }
            Rule::cypher => {
                let cypher_str = expr.as_str();
                Expression::Escape(cypher_str.to_string())
            }
            Rule::float => Expression::Literal(Value::Float(Float::from(
                expr.as_str().parse::<f64>().unwrap(),
            ))),
            Rule::integer => Expression::Literal(Value::Integer(Integer::from(
                expr.as_str().parse::<i64>().unwrap(),
            ))),
            Rule::bool => match expr.as_str() {
                "TRUE" => Expression::Literal(Value::Boolean(true)),
                "FALSE" => Expression::Literal(Value::Boolean(false)),
                _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
            },
            Rule::text => Expression::Literal(Value::String(expr.as_str().to_string())),
            Rule::value => {
                let value = Value::from_pair(expr)?;
                Expression::Literal(value)
            }
            _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
        };

        expressions.push(expr);
        Ok(expressions.len() - 1)
    }
}

impl<'i> FromPair<'i> for Value {
    fn from_pair(pair: Pair<'i, Rule>) -> Result<Self, ASTError> {
        match pair.as_rule() {
            Rule::float => Ok(Value::Float(Float::from(
                pair.as_str().parse::<f64>().unwrap(),
            ))),
            Rule::integer => Ok(Value::Integer(Integer::from(
                pair.as_str().parse::<i64>().unwrap(),
            ))),
            Rule::bool => match pair.as_str() {
                "TRUE" => Ok(Value::Boolean(true)),
                "FALSE" => Ok(Value::Boolean(false)),
                _ => Err(ASTError::UnexpectedRule(pair.as_rule())),
            },
            Rule::text => Ok(Value::String(pair.as_str().to_string())),
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

pub trait AlgebraFold {
    type Output;
    type Error: From<ASTError>;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error>;

    fn fold_print(
        &mut self,
        print: &StorePrint,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    fn fold_project(
        &mut self,
        project: &Project,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;
}

impl AST {
    pub fn accept<F: AlgebraFold>(&self, folder: &mut F) -> Result<F::Output, F::Error> {
        self.fold_algebra(self.root, folder)
    }

    fn fold_algebra<F: AlgebraFold>(
        &self,
        index: AlgRef,
        folder: &mut F,
    ) -> Result<F::Output, F::Error> {
        if let Some(alg) = self.algebra.get(index) {
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
