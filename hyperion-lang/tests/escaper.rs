mod common;

#[cfg(test)]
mod escaper_parse {
    use rstest::rstest;

    use crate::common::{assert_parse_equal, assert_parse_err};

    #[rstest]
    #[case("SQL(BETWEEN 0 AND 1)")]
    #[case("SQL(EXP(x))")]
    fn parse_valid_sql_escape(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::sql, value);
    }

    #[rstest]
    #[case("SQL(")]
    #[case("SQL)")]
    #[case("SQL(EXP(x)")]
    #[case("SQLEXP(x))")]
    #[case("")]
    fn parse_invalid_sql_escape(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::sql, value);
    }
}
