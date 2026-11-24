use hyperion_value::Value;

use crate::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::ASTError,
    expression::{BinExpOp, UnExpOp},
};

pub trait AlgebraFolder {
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

    fn fold_window(
        &mut self,
        window: &Window,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;
}

pub trait ExpressionFolder {
    type Output;
    type Error: From<ASTError>;

    fn fold_escape(&mut self, escape: &String) -> Result<Self::Output, Self::Error>;

    fn fold_literal(&mut self, literal: &Value) -> Result<Self::Output, Self::Error>;

    fn fold_target(&mut self, target: &String) -> Result<Self::Output, Self::Error>;

    fn fold_unary_expression(
        &mut self,
        op: &UnExpOp,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    fn fold_binary_expression(
        &mut self,
        op: &BinExpOp,
        input_left: Self::Output,
        input_right: Self::Output,
    ) -> Result<Self::Output, Self::Error>;
}
