mod common;

#[cfg(test)]
mod value_parse {
    use hyperion_lang::parser::LangParser;
    use pest::Parser;
    use rstest::rstest;

    use crate::common::{assert_parse_equal, assert_parse_err};

    #[rstest]
    #[case("TRUE")]
    #[case("FALSE")]
    fn parse_valid_bool(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::BOOL, value, value);
    }

    #[rstest]
    #[case("tRue")]
    #[case("fAlse")]
    #[case("")]
    fn parse_invalid_bool(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::BOOL, value);
    }

    #[test]
    fn parse_text() {
        //assert_parse_equal(hyperion_lang::parser::Rule::TEXT, "somerandomtext");
    }

    #[rstest]
    #[case("0", "0")]
    #[case("01", "01")]
    #[case("10", "10")]
    #[case("0.0", "0")]
    fn parse_valid_integer(#[case] value: &str, #[case] expected: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::INTEGER, value, expected);
    }

    #[rstest]
    #[case("")]
    fn parse_invalid_integer(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::INTEGER, value);
    }

    #[rstest]
    #[case("0.0")]
    #[case("1.0")]
    #[case("0.1")]
    #[case("10.0")]
    #[case("0.01")]
    fn parse_valid_float(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::FLOAT, value, value);
    }

    #[rstest]
    #[case("")]
    fn parse_invalid_float(#[case] value: &str) {
        assert_parse_err(hyperion_lang::parser::Rule::FLOAT, value);
    }
}
