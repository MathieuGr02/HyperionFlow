use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan},
    ast::{AlgebraFold, ExpressionFold},
    expression::{BinExpOp, UnExpOp},
};
use hyperion_value::Value;

use crate::stores::postgres::util::SQLError;

pub struct SQLQuery {
    pub project: Vec<SQLExpression>,
    pub filter: Vec<SQLExpression>,
    pub source: SQLSource,
}

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
                filter: Vec::new(),
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
        todo!()
    }

    fn fold_project(
        &mut self,
        project: &Project,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let expr = project.expression.fold(&mut SQLExpressionCompiler)?;
        todo!()
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let expr = filter.expression.fold(&mut SQLExpressionCompiler)?;
        todo!()
    }
}

#[derive(Debug)]
pub enum SQLExpression {
    Raw(String),
    Literal(Value),
    Identifier(String),
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
        todo!()
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
