use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan},
    ast::{AlgebraAST, AlgebraFold},
};

use crate::stores::postgres::util::SQLError;

pub struct SQLQuery {
    pub project: Vec<String>,
    pub source: SQLSource,
}

pub struct SQLSource {
    table: String,
}

pub struct SQLCompiler {}

impl AlgebraFold for SQLCompiler {
    type Output = SQLQuery;
    type Error = SQLError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        match scan {
            StoreScan::SQLScan(sql_scan) => Ok(SQLQuery {
                project: Vec::new(),
                source: SQLSource { table: sql_scan.table.to_owned() }
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
        project.
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
