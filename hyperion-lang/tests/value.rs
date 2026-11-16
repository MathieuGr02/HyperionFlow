mod common;

#[cfg(test)]
mod value_parse {
    use hyperion_lang::parser::{LangParser, Rule::bool};
    use pest::Parser;
    use rstest::rstest;

    use crate::common::{assert_parse_equal, assert_parse_err};

    #[rstest]
    #[case("TRUE")]
    #[case("FALSE")]
    fn parse_valid_bool(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::bool, value);
    }

    #[rstest]
    #[case("tRue")]
    #[case("fAlse")]
    #[case("")]
    fn parse_invalid_bool(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::bool, value);
    }

    #[test]
    fn parse_text() {
        assert_parse_equal(hyperion_lang::parser::Rule::text, "somerandomtext");
    }

    #[rstest]
    #[case("0")]
    #[case("01")]
    #[case("10")]
    fn parse_valid_integer(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::integer, value);
    }

    #[rstest]
    #[case("0.0")]
    #[case("")]
    fn parse_invalid_integer(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::integer, value);
    }

    #[rstest]
    #[case("0.0")]
    #[case("1.0")]
    #[case("0.1")]
    #[case("10.0")]
    #[case("0.01")]
    fn parse_valid_float(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::float, value);
    }

    #[rstest]
    #[case("")]
    fn parse_invalid_float(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::float, value);
    }
}
