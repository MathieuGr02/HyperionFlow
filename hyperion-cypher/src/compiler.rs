use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::{ASTError, AlgebraFold, ExpressionFold},
    expression::{BinExpOp, UnExpOp},
};
use hyperion_value::Value;
use neo4rs::Query;
use thiserror::Error;

use crate::value::CypherError;

pub struct CypherQueryCompiler;

pub struct CypherQuery;

impl CypherQuery {
    pub fn build(&self) -> Query {
        Query::new(String::new())
    }
}

impl AlgebraFold for CypherQueryCompiler {
    type Output = CypherQuery;
    type Error = CypherError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        todo!()
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
        todo!()
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn fold_window(
        &mut self,
        window: &Window,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

pub struct CypherExpressionCompiler;

pub struct CypherExpression;

impl ExpressionFold for CypherExpressionCompiler {
    type Output = CypherExpression;
    type Error = CypherError;

    fn fold_escape(&mut self, escape: &String) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn fold_literal(&mut self, literal: &Value) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn fold_target(&mut self, target: &String) -> Result<Self::Output, Self::Error> {
        todo!()
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
