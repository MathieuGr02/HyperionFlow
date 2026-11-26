use hyperion_lang::{
    algebra::{Filter, Project, ProjectExpression, StorePrint, StoreScan, Window},
    expression::{BinExpOp, UnExpOp},
    folder::{AlgebraFolder, ExpressionFolder},
};
use hyperion_value::Value;
use tracing::warn;

use crate::{database_type::DatabaseType, stores::postgres::util::SQLError};

#[derive(Debug)]
pub struct SQLQuery {
    pub insert: Option<SQLTarget>,
    pub project: Vec<SQLProject>,
    pub filter: Option<SQLExpression>,
    pub source: SQLSource,
}

#[derive(Debug)]
pub struct SQLProject {
    expr: SQLExpression,
    alias: Option<String>,
}

impl SQLQuery {
    pub fn get_target_columns(&self) -> Vec<String> {
        let mut columns = Vec::new();
        for project_expr in self.project.iter() {
            match project_expr.alias.as_ref() {
                Some(alias) => columns.push(alias.to_owned()),
                None => match &project_expr.expr {
                    SQLExpression::Identifier(identifier) => columns.push(identifier.to_owned()),
                    _ => (), //return Err(SQLError::Custom("Project expression not identifier")),
                },
            }
        }

        columns
    }
}

impl SQLQuery {
    pub fn build<DB: DatabaseType>(&self, db: &DB) -> Result<String, SQLError> {
        let rows: Vec<String> = self
            .project
            .iter()
            .map(|p| {
                let expr = p.expr.build(db);
                if let Some(alias) = p.alias.as_ref() {
                    format!("{} AS {}", expr, alias)
                } else {
                    format!("{expr}")
                }
            })
            .collect();
        let rows = rows.join(", ");
        let rows = rows.trim();
        if rows.is_empty() {
            return Err(SQLError::Custom("Empty select clause".to_string()));
        }

        let select_clause = format!("SELECT {rows}");

        let from_clause = format!("FROM {}", self.source.table);

        let mut where_clause = String::new();
        if let Some(filter) = self.filter.as_ref() {
            let filter = filter.build(db);
            where_clause = format!("WHERE {filter}");
        }

        let mut insert_clause = String::new();
        if let Some(target) = self.insert.as_ref() {
            let values = target.columns.join(", ");
            insert_clause = format!("INSERT INTO {} ({})", target.table, values);
        }

        Ok(
            format!("{insert_clause} {select_clause} {from_clause} {where_clause}")
                .trim()
                .to_string(),
        )
    }
}

#[derive(Debug)]
pub struct SQLSource {
    table: String,
}

#[derive(Debug)]
pub struct SQLTarget {
    table: String,
    columns: Vec<String>,
}

pub struct SQLQueryCompiler;

impl AlgebraFolder for SQLQueryCompiler {
    type Output = SQLQuery;
    type Error = SQLError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        match scan {
            StoreScan::SQLScan(sql_scan) => Ok(SQLQuery {
                insert: None,
                project: Vec::new(),
                filter: None,
                source: SQLSource {
                    table: sql_scan.table.to_owned(),
                },
            }),
            _ => Err(SQLError::UnsupportedType("Invalid scan".to_string())),
        }
    }

    fn fold_print(
        &mut self,
        print: &StorePrint,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let columns = input.get_target_columns();
        match print {
            StorePrint::SQLPrint(sql_print) => {
                let target = SQLTarget {
                    table: sql_print.table.clone(),
                    columns,
                };
                input.insert = Some(target);
                Ok(input)
            }
            _ => Err(SQLError::UnsupportedType("Invalid print".to_string())),
        }
    }

    fn fold_project(
        &mut self,
        project: &Project,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let mut expr_compiler = SQLExpressionCompiler;
        dbg!(&project);
        for expr in project.expression.iter() {
            let expr_res = expr.expression.fold(&mut expr_compiler)?;
            let project_expr = SQLProject {
                expr: expr_res,
                alias: expr.alias.clone(),
            };
            input.project.push(project_expr);
        }
        dbg!(&input);
        Ok(input)
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let expr = filter.expression.fold(&mut SQLExpressionCompiler)?;
        input.filter = Some(expr);
        Ok(input)
    }

    fn fold_window(
        &mut self,
        window: &Window,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        dbg!(&window);
        let expr = match window {
            // Count window only important for the runtime
            Window::CountWindow {
                column: opt_column,
                size: _,
            } => {
                if let Some(column) = opt_column {
                    // column >= X1
                    let left = SQLExpression::Binary {
                        op: String::from(">="),
                        left: SQLExpression::Identifier(column.to_owned()).into(),
                        right: SQLExpression::PlaceHolder(1).into(),
                    };
                    // column <= X2
                    let right = SQLExpression::Binary {
                        op: String::from("<="),
                        left: SQLExpression::Identifier(column.to_owned()).into(),
                        right: SQLExpression::PlaceHolder(2).into(),
                    };
                    // column >= X1 AND column <= X2 => column in range [X1, X2]
                    SQLExpression::Binary {
                        op: String::from("AND"),
                        left: left.into(),
                        right: right.into(),
                    }
                } else {
                    warn!("No column set for count window. Skipping parsing of count window.");
                    return Ok(input);
                }
            }
            // Tumble window translated into range query
            Window::TumbleWindow {
                column: opt_column,
                size: _,
            } => {
                if let Some(column) = opt_column {
                    // column >= X1
                    let left = SQLExpression::Binary {
                        op: String::from(">="),
                        left: SQLExpression::Identifier(column.to_owned()).into(),
                        right: SQLExpression::PlaceHolder(1).into(),
                    };
                    // column <= X2
                    let right = SQLExpression::Binary {
                        op: String::from("<="),
                        left: SQLExpression::Identifier(column.to_owned()).into(),
                        right: SQLExpression::PlaceHolder(2).into(),
                    };
                    // column >= X1 AND column <= X2 => column in range [X1, X2]
                    SQLExpression::Binary {
                        op: String::from("AND"),
                        left: left.into(),
                        right: right.into(),
                    }
                } else {
                    warn!("No column set for tumble window. Skipping parsing of tumble window.");
                    return Ok(input);
                }
            }
        };
        // Swap root sql expression by combining former root with window through AND clause
        if let Some(filter) = input.filter.take() {
            let binary = SQLExpression::Binary {
                op: String::from("AND"),
                left: SQLExpression::Parentheses(filter.into()).into(),
                right: SQLExpression::Parentheses(expr.into()).into(),
            };
            input.filter = Some(binary)
        } else {
            input.filter = Some(expr);
        }
        Ok(input)
    }
}

#[derive(Debug)]
pub enum SQLExpression {
    PlaceHolder(usize),
    Raw(String),
    Literal(Value),
    Identifier(String),
    Parentheses(Box<SQLExpression>),
    Unary {
        op: String,
        input: Box<SQLExpression>,
    },
    Binary {
        op: String,
        left: Box<SQLExpression>,
        right: Box<SQLExpression>,
    },
}

impl SQLExpression {
    pub fn build<DB: DatabaseType>(&self, db: &DB) -> String {
        match self {
            SQLExpression::PlaceHolder(counter) => format!("{}{}", DB::PLACEHOLDER, counter),
            SQLExpression::Raw(string) => string.to_owned(),
            SQLExpression::Literal(value) => match value {
                Value::Null => "NULL".to_string(),
                Value::Boolean(bool) => match bool {
                    true => "TRUE".to_string(),
                    false => "FALSE".to_string(),
                },
                Value::Bytes(bytes) => format!("B'{bytes}'"),
                Value::Integer(integer) => format!("{integer}"),
                Value::Float(float) => format!("{float}"),
                Value::String(string) => format!("\"{string}\""),
                // Unsupported currently
                Value::Timestamp(timestamp) => format!(""),
                Value::Array(values) => format!(""),
                Value::Map(map) => format!(""),
                Value::Node(node) => format!(""),
                Value::Edge(edge) => format!(""),
            },
            SQLExpression::Parentheses(expr) => {
                let inner = expr.build(db);
                format!("({inner})")
            }
            SQLExpression::Identifier(identifier) => identifier.to_owned(),
            SQLExpression::Unary { op, input } => {
                let inner = input.build(db);
                format!("{op}({inner})")
            }
            SQLExpression::Binary { op, left, right } => {
                let left_expr = left.build(db);
                let right_expr = right.build(db);
                format!("{left_expr} {op} {right_expr}")
            }
        }
    }
}

#[derive(Debug)]
pub struct SQLExpressionCompiler;

impl ExpressionFolder for SQLExpressionCompiler {
    type Output = SQLExpression;
    type Error = SQLError;

    fn fold_escape(&mut self, escape: &String) -> Result<Self::Output, Self::Error> {
        Ok(SQLExpression::Raw(escape.to_owned()))
    }

    fn fold_literal(&mut self, literal: &Value) -> Result<Self::Output, Self::Error> {
        Ok(SQLExpression::Literal(literal.clone()))
    }

    fn fold_target(&mut self, target: &String) -> Result<Self::Output, Self::Error> {
        Ok(SQLExpression::Identifier(target.to_owned()))
    }

    fn fold_unary_expression(
        &mut self,
        op: &UnExpOp,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let op_str = match op {
            UnExpOp::Abs => "ABS",
            UnExpOp::Avg => "AVG",
            UnExpOp::Ceiling => "CEILING",
            UnExpOp::Count => "COUNT",
            UnExpOp::Exp => "EXP",
            UnExpOp::Floor => "FLOOR",
            UnExpOp::Max => "MAX",
            UnExpOp::Min => "MIN",
            UnExpOp::Sign => "SIGN",
            UnExpOp::Sqrt => "SQRT",
            UnExpOp::Square => "SQUARE",
            UnExpOp::Sum => "SUM",
        };

        Ok(SQLExpression::Unary {
            op: op_str.to_string(),
            input: Box::new(input),
        })
    }

    fn fold_binary_expression(
        &mut self,
        op: &BinExpOp,
        input_left: Self::Output,
        input_right: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let op_str = match op {
            BinExpOp::Rename => "AS",
            BinExpOp::Eq => "==",
            BinExpOp::Geq => ">=",
            BinExpOp::Leq => "<=",
            BinExpOp::Between => todo!(),
        };

        Ok(SQLExpression::Binary {
            op: op_str.to_string(),
            left: Box::new(input_left),
            right: Box::new(input_right),
        })
    }
}
