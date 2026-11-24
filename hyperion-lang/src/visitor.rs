use crate::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::ASTError,
};

pub trait AlgebraVisitor {
    type Output;
    type Error: From<ASTError>;

    fn visit_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error>;
    fn visit_print(&mut self, print: &StorePrint) -> Result<Self::Output, Self::Error>;
    fn visit_project(&mut self, project: &Project) -> Result<Self::Output, Self::Error>;
    fn visit_filter(&mut self, filter: &Filter) -> Result<Self::Output, Self::Error>;
    fn visit_window(&mut self, window: &Window) -> Result<Self::Output, Self::Error>;
}

pub trait AlgebraMutVisitor {
    type Output;
    type Error: From<ASTError>;

    fn visit_scan_mut(&mut self, scan: &mut StoreScan) -> Result<Self::Output, Self::Error>;
    fn visit_print_mut(&mut self, print: &mut StorePrint) -> Result<Self::Output, Self::Error>;
    fn visit_project_mut(&mut self, project: &mut Project) -> Result<Self::Output, Self::Error>;
    fn visit_filter_mut(&mut self, filter: &mut Filter) -> Result<Self::Output, Self::Error>;
    fn visit_window_mut(&mut self, window: &mut Window) -> Result<Self::Output, Self::Error>;
}
