use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::ASTError,
    expression::{BinExpOp, UnExpOp},
    folder::{AlgebraFolder, ExpressionFolder},
};
use hyperion_value::Value;
use mongodb::bson::Document;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MQLError {
    #[error("{0}")]
    ASTError(#[from] ASTError),
}

pub struct MQLQuery;

impl MQLQuery {
    pub fn build(&self) -> Document {
        Document::new()
    }
}

pub struct MQLQueryCompiler;

impl AlgebraFolder for MQLQueryCompiler {
    type Output = MQLQuery;
    type Error = MQLError;

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

pub struct MQLExpression;

pub struct MQLExpressionCompiler;

impl ExpressionFolder for MQLExpressionCompiler {
    type Output = MQLExpression;
    type Error = MQLError;

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
