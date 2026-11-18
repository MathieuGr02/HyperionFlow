use hyperion_lang::{
    algebra::{Filter, Project, Scan},
    ast::{AST, AlgebraVisitor},
    compiler::Compiler,
};

use crate::stores::postgres::util::SQLError;

pub struct SQLQuery {
    query: String,
}

pub struct SQLVisitor {}

impl AlgebraVisitor for SQLVisitor {
    type Output = SQLQuery;
    type Error = SQLError;

    fn visit_scan(&mut self, scan: &Scan) -> Result<Self::Output, Self::Error> {
        match scan {
            Scan::SQLScan(sql_scan) => {
                format!("FROM {}", sql_scan.table);
            }
            _ => return Err(SQLError::Custom("Invalid scan".to_string())),
        }
    }

    fn visit_project(&mut self, project: &Project) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn visit_filter(&mut self, filter: &Filter) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}
