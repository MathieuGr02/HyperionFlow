mod common;

#[cfg(test)]
mod filter_parse {
    use rstest::rstest;

    use crate::common::assert_parse_equal;

    #[rstest]
    #[case("FILTER SQL(BETWEEN 0 AND 1)")]
    fn parse_filter(#[case] value: &str) {
        assert_parse_equal(hyperion_lang::parser::Rule::filter, value, value);
    }
}
