use pest_ast::FromPest;

use crate::parser::Rule;

#[derive(Debug, FromPest)]
#[pest_ast(rule(Rule::sql))]
pub struct SQLEscape {
    pub sql: String,
}
