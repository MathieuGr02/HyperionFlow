use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "lang.pest"]
pub struct LangParser {}
