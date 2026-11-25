use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    folder::AlgebraFolder,
};

use crate::{
    config::trigger::{CountTrigger, TriggerType},
    runtime::engine::EngineError,
};

pub struct RuntimeCompiler;

pub struct RuntimeAlgebra {
    pub trigger_type: Option<TriggerType>,
}

impl AlgebraFolder for RuntimeCompiler {
    type Output = RuntimeAlgebra;
    type Error = EngineError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        Ok(RuntimeAlgebra { trigger_type: None })
    }

    fn fold_print(
        &mut self,
        print: &StorePrint,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }

    fn fold_project(
        &mut self,
        project: &Project,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        Ok(input)
    }

    fn fold_window(
        &mut self,
        window: &Window,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let trigger = match window {
            Window::CountWindow { size, column: _ } => {
                TriggerType::CountTrigger(CountTrigger::new(size.as_i64() as usize))
            }
            Window::TumbleWindow { column, size } => todo!(),
        };
        input.trigger_type = Some(trigger);
        Ok(input)
    }
}
