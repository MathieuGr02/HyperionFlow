use pest_derive::Parser;

#[derive(Debug, Parser)]
#[grammar = "language/lang.pest"]
pub struct LangParser {}
