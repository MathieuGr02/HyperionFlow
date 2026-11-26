use pest::{Parser, iterators::Pair};
use thiserror::Error;

use crate::algebra::{
    CypherPrint, CypherScan, MQLPrint, MQLScan, ProjectExpression, SQLPrint, SQLScan, StorePrint,
    StoreScan, Window,
};
use crate::arena_tree::{ArenaTree, Ref};
use crate::expression::UnExpOp;
use crate::folder::{AlgebraFolder, ExpressionFolder};
use crate::visitor::{AlgebraMutVisitor, AlgebraVisitor};
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

#[derive(Debug)]
pub struct LogicalExpression {
    tree: ArenaTree<Expression>,
}

impl LogicalExpression {
    /// Parse logical expression as a query into a [`LogicalExpression`] type
    /// The expression is internally represented by an arena AST
    pub fn parse(expr: &str) -> Result<Self, ASTError> {
        let pairs = LangParser::parse(Rule::expr, expr)
            .map_err(|e| ASTError::ParsingError(format!("Expression parsing error {e}")))?;

        let pair = pairs.into_iter().next().unwrap();

        Self::parse_pair(pair)
    }

    pub fn parse_pair(pair: Pair<'_, Rule>) -> Result<Self, ASTError> {
        let mut logical_expr = Self {
            tree: ArenaTree::new(),
        };

        let root = logical_expr.build_expression(pair)?;
        logical_expr.tree.set_root(root);

        Ok(logical_expr)
    }

    /// Internal recursion function to recursively build the expression tree.
    /// The tree is build bottom up by having the top expressions recurse to the leaf nodes
    /// The leaf nodes are then build into [`Expression`], returing their tree node refernce for the parent
    fn build_expression(&mut self, expr: Pair<'_, Rule>) -> Result<Ref, ASTError> {
        let expr_ref = match expr.as_rule() {
            Rule::expr => {
                let mut inner = expr.into_inner();
                self.build_expression(inner.next().unwrap())?
            }

            Rule::escaper => {
                let mut inner = expr.into_inner();
                let _identifier = inner.next().unwrap();
                let content = inner.next().unwrap().as_str();
                let escape = Expression::Escape(content.to_owned());
                self.tree.add(escape)
            }
            Rule::unary_math_functions => {
                let mut inner = expr.into_inner();

                let callable = inner.next().unwrap();
                let op = UnExpOp::from_pair(callable)?;
                let inner_expr = inner.next().unwrap();
                let inner_expr = self.build_expression(inner_expr)?;

                let unary = Expression::UnaryExpression {
                    op,
                    value: inner_expr,
                };
                self.tree.add(unary)
            }
            Rule::TARGET => {
                let target = Expression::Target(expr.as_str().to_string());
                self.tree.add(target)
            }
            // Primitive types
            Rule::FLOAT => {
                let literal = Expression::Literal(Value::Float(Float::from(
                    expr.as_str().parse::<f64>().unwrap(),
                )));
                self.tree.add(literal)
            }
            Rule::INTEGER => {
                let literal = Expression::Literal(Value::Integer(Integer::from(
                    expr.as_str().parse::<i64>().unwrap(),
                )));
                self.tree.add(literal)
            }
            Rule::BOOL => {
                let literal = match expr.as_str() {
                    "TRUE" => Expression::Literal(Value::Boolean(true)),
                    "FALSE" => Expression::Literal(Value::Boolean(false)),
                    _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
                };
                self.tree.add(literal)
            }
            Rule::TEXT => {
                let literal = Expression::Literal(Value::String(expr.as_str().to_string()));
                self.tree.add(literal)
            }
            Rule::VALUE => {
                let value = Value::from_pair(expr)?;
                let literal = Expression::Literal(value);
                self.tree.add(literal)
            }
            _ => return Err(ASTError::UnexpectedRule(expr.as_rule())),
        };

        Ok(expr_ref)
    }
}

#[derive(Debug)]
pub struct LogicalPlan {
    tree: ArenaTree<Algebra>,
}

impl LogicalPlan {
    /// Parse a query into a [`LogicalPlan`] type
    /// The plan is internally represented by an arena AST
    pub fn parse(query: &str) -> Result<Self, ASTError> {
        let pairs = LangParser::parse(Rule::query, query)
            .map_err(|e| ASTError::ParsingError(format!("Algebra parsing error {e}")))?;

        let query = pairs.into_iter().next().unwrap();
        let algebra_query = query
            .into_inner()
            .into_iter()
            .find(|p| p.as_rule() == Rule::algebra)
            .ok_or(ASTError::MissingRule(Rule::algebra))?;

        let mut logical_plan = Self {
            tree: ArenaTree::new(),
        };

        logical_plan.build_ast(algebra_query)?;

        Ok(logical_plan)
    }

    fn build_ast(&mut self, query: Pair<'_, Rule>) -> Result<(), ASTError> {
        let mut current = Ref::zero();
        for operation in query.into_inner() {
            current = self.build_algebra(operation, current)?;
        }

        self.tree.set_root(current);

        Ok(())
    }

    fn build_algebra(&mut self, pair: Pair<'_, Rule>, input: Ref) -> Result<Ref, ASTError> {
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
                let mut project_expressions = Vec::new();
                for inner in pair.into_inner().into_iter() {
                    let pair = inner.into_inner().next().unwrap();
                    let project_expr = match pair.as_rule() {
                        Rule::TARGET => {
                            let expr_ast = LogicalExpression::parse_pair(pair)?;
                            ProjectExpression {
                                expression: expr_ast,
                                alias: None,
                            }
                        }
                        Rule::project_rename => {
                            let mut rename_inner = pair.into_inner();
                            let project_expr = rename_inner.next().unwrap();
                            let target = rename_inner.next().unwrap();

                            let project_expr = LogicalExpression::parse_pair(project_expr)?;
                            let target = target.as_str();

                            ProjectExpression {
                                expression: project_expr,
                                alias: Some(target.to_string()),
                            }
                        }
                        _ => return Err(ASTError::UnexpectedRule(pair.as_rule())),
                    };

                    project_expressions.push(project_expr);
                }

                Algebra::Unary {
                    alg: UnaryAlgebra::Project(Project {
                        expression: project_expressions,
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

        let index = self.tree.add(alg);
        Ok(index)
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

impl LogicalExpression {
    /// Fold [`LogicalExpression`] using the provided `folder`
    pub fn fold<E: ExpressionFolder>(&self, folder: &mut E) -> Result<E::Output, E::Error> {
        if let Some(root) = self.tree.get_root() {
            self.fold_expression(*root, folder)
        } else {
            Err(ASTError::ParsingError(String::from("No root node set for expression tree")).into())
        }
    }

    /// Internal recurse function to recursively fold the [`LogicalExpression`] tree.
    /// The folding occurs bottom up.
    fn fold_expression<E: ExpressionFolder>(
        &self,
        index: Ref,
        folder: &mut E,
    ) -> Result<E::Output, E::Error> {
        if let Some(expr) = self.tree.get(index) {
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
    pub fn accept<V: AlgebraVisitor>(&self, visitor: &mut V) -> Result<V::Output, V::Error> {
        if let Some(root) = self.tree.get_root() {
            self.visit_algebra(*root, visitor)
        } else {
            Err(ASTError::ParsingError(String::from("No root node set for algebra tree")).into())
        }
    }

    pub fn visit_algebra<V: AlgebraVisitor>(
        &self,
        index: Ref,
        visitor: &mut V,
    ) -> Result<V::Output, V::Error> {
        if let Some(alg) = self.tree.get(index) {
            match alg {
                Algebra::Print { print, input } => {
                    self.visit_algebra(*input, visitor)?;
                    visitor.visit_print(print)
                }
                Algebra::Scan(store_scan) => visitor.visit_scan(store_scan),
                Algebra::Unary { alg, input } => {
                    self.visit_algebra(*input, visitor)?;
                    match alg {
                        UnaryAlgebra::Filter(filter) => visitor.visit_filter(filter),
                        UnaryAlgebra::Project(project) => visitor.visit_project(project),
                        UnaryAlgebra::Window(window) => visitor.visit_window(window),
                    }
                }
                Algebra::Binary { alg, left, right } => todo!(),
            }
        } else {
            Err(ASTError::ParsingError(format!("Invalid index {}", index)).into())
        }
    }

    pub fn accept_mut<V: AlgebraMutVisitor>(
        &mut self,
        visitor: &mut V,
    ) -> Result<V::Output, V::Error> {
        if let Some(root) = self.tree.get_root() {
            self.visit_algebra_mut(*root, visitor)
        } else {
            Err(ASTError::ParsingError(String::from("No root node set for algebra tree")).into())
        }
    }

    fn visit_algebra_mut<V: AlgebraMutVisitor>(
        &mut self,
        index: Ref,
        visitor: &mut V,
    ) -> Result<V::Output, V::Error> {
        let refs = match self.tree.get(index) {
            Some(Algebra::Print { print: _, input }) => {
                vec![input.clone()]
            }
            Some(Algebra::Scan(_)) => vec![],
            Some(Algebra::Unary { alg: _, input }) => vec![input.clone()],
            Some(Algebra::Binary {
                alg: _,
                left,
                right,
            }) => vec![left.clone(), right.clone()],
            None => return Err(ASTError::ParsingError(format!("Invalid index {}", index)).into()),
        };

        for ref_i in refs {
            self.visit_algebra_mut(ref_i, visitor)?;
        }

        match self.tree.get_mut(index) {
            Some(Algebra::Print { print, input: _ }) => visitor.visit_print_mut(print),
            Some(Algebra::Scan(store_scan)) => visitor.visit_scan_mut(store_scan),
            Some(Algebra::Unary { alg, input: _ }) => match alg {
                UnaryAlgebra::Filter(filter) => visitor.visit_filter_mut(filter),
                UnaryAlgebra::Project(project) => visitor.visit_project_mut(project),
                UnaryAlgebra::Window(window) => visitor.visit_window_mut(window),
            },
            Some(Algebra::Binary {
                alg,
                left: _,
                right: _,
            }) => todo!(),
            None => todo!(),
        }
    }

    pub fn fold<F: AlgebraFolder>(&self, folder: &mut F) -> Result<F::Output, F::Error> {
        if let Some(root) = self.tree.get_root() {
            self.fold_algebra(*root, folder)
        } else {
            Err(ASTError::ParsingError(String::from("No root node set for algebra tree")).into())
        }
    }

    fn fold_algebra<F: AlgebraFolder>(
        &self,
        index: Ref,
        folder: &mut F,
    ) -> Result<F::Output, F::Error> {
        if let Some(alg) = self.tree.get(index) {
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
            Err(ASTError::ParsingError(format!("Invalid index {}", index)).into())
        }
    }
}
