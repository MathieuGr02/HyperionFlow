use hyperion_lang::{
    algebra::{Filter, Project},
    ast::{AST, AlgebraFold, AlgebraVisitor},
};

use crate::stores::postgres::util::SQLError;

pub struct SQLQuery {
    query: String,
}

pub struct SQLCompiler {}

impl AlgebraFold for SQLCompiler {
    type Output = SQLQuery;
    type Error = SQLError;

    fn fold_scan(
        &mut self,
        scan: &hyperion_lang::algebra::StoreScan,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn fold_print(
        &mut self,
        print: &hyperion_lang::algebra::StorePrint,
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
}
