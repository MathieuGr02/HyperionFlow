use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    visitor::AlgebraMutVisitor,
};

use crate::runtime::engine::EngineError;

pub struct RuntimeVisitor {
    pub default_time_column: String,
}

impl AlgebraMutVisitor for RuntimeVisitor {
    type Output = ();
    type Error = EngineError;

    fn visit_scan_mut(&mut self, scan: &mut StoreScan) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn visit_print_mut(&mut self, print: &mut StorePrint) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn visit_project_mut(&mut self, project: &mut Project) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn visit_filter_mut(&mut self, filter: &mut Filter) -> Result<Self::Output, Self::Error> {
        Ok(())
    }

    fn visit_window_mut(&mut self, window: &mut Window) -> Result<Self::Output, Self::Error> {
        match window {
            Window::CountWindow { column, size } => {
                if column.is_none() {
                    column.replace(self.default_time_column);
                }
            }
            Window::TumbleWindow { column, size } => {
                if column.is_none() {
                    column.replace(self.default_time_column);
                }
            }
        }
        todo!()
    }
}
