mod common;

#[cfg(test)]
mod math_parse {
    use crate::common::{assert_parse_equal, assert_parse_err};

    #[test]
    fn parse_math_op_add() {
        assert_parse_equal(hyperion_lang::parser::Rule::add, "+");
        assert_parse_err(hyperion_lang::parser::Rule::add, "-");
        assert_parse_err(hyperion_lang::parser::Rule::add, "");
    }

    #[test]
    fn parse_math_op_sub() {
        assert_parse_equal(hyperion_lang::parser::Rule::sub, "-");
        assert_parse_err(hyperion_lang::parser::Rule::sub, "+");
        assert_parse_err(hyperion_lang::parser::Rule::sub, "");
    }

    #[test]
    fn parse_math_op_mul() {
        assert_parse_equal(hyperion_lang::parser::Rule::mul, "*");
        assert_parse_err(hyperion_lang::parser::Rule::mul, "+");
        assert_parse_err(hyperion_lang::parser::Rule::mul, "");
    }

    #[test]
    fn parse_math_op_div() {
        assert_parse_equal(hyperion_lang::parser::Rule::div, "/");
        assert_parse_err(hyperion_lang::parser::Rule::div, "+");
        assert_parse_err(hyperion_lang::parser::Rule::div, "");
    }
}
