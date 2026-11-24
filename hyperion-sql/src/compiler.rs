use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::{AlgebraFold, ExpressionFold},
    expression::{BinExpOp, UnExpOp},
};
use hyperion_value::Value;

use crate::{
    compiler::SQLExpression::PlaceHolder, database_type::DatabaseType,
    stores::postgres::util::SQLError,
};

#[derive(Debug)]
pub struct SQLQuery {
    pub project: Vec<SQLExpression>,
    // TODO: alter to vector or provide method to combine expressions
    pub filter: Option<SQLExpression>,
    pub source: SQLSource,
}

impl SQLQuery {
    pub fn build<DB: DatabaseType>(&self, db: &DB) -> String {
        let rows: Vec<String> = self.project.iter().map(|p| p.build(db)).collect();
        let rows = rows.join(", ");
        let select_clause = format!("SELECT {rows}");

        let from_clause = format!("FROM {}", self.source.table);

        let mut where_clause = String::new();
        if let Some(filter) = self.filter.as_ref() {
            let filter = filter.build(db);
            where_clause = format!("WHERE {filter}");
        }

        format!("{select_clause} {from_clause} {where_clause}")
            .trim()
            .to_string()
    }
}

#[derive(Debug)]
pub struct SQLSource {
    table: String,
}

pub struct SQLQueryCompiler;

impl AlgebraFold for SQLQueryCompiler {
    type Output = SQLQuery;
    type Error = SQLError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        match scan {
            StoreScan::SQLScan(sql_scan) => Ok(SQLQuery {
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
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(input)
        //todo!()
    }

    fn fold_project(
        &mut self,
        project: &Project,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let mut expr_compiler = SQLExpressionCompiler;
        for expr in project.expression.iter() {
            let expr_res = expr.fold(&mut expr_compiler)?;
            input.project.push(expr_res);
        }
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
        let expr = match window {
            Window::CountWindow { size: _ } => return Ok(input),
            Window::TumbleWindow { column: _, size: _ } => SQLExpression::Binary {
                op: String::from("BETWEEN"),
                left: PlaceHolder.into(),
                right: PlaceHolder.into(),
            },
        };
        input.filter = Some(expr);
        Ok(input)
    }
}

#[derive(Debug)]
pub enum SQLExpression {
    PlaceHolder,
    Raw(String),
    Literal(Value),
    Identifier(String),
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
            SQLExpression::PlaceHolder => DB::PLACEHOLDER.to_owned(),
            SQLExpression::Raw(string) => string.to_owned(),
            SQLExpression::Literal(value) => todo!(),
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

impl ExpressionFold for SQLExpressionCompiler {
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
        todo!()
    }
}
