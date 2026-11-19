use hyperion_lang::parser::{LangParser, Rule};
use pest::Parser;

pub fn assert_parse_err(rule: Rule, value: &str) {
    let res = LangParser::parse(rule, value);
    assert!(res.is_err());
}

pub fn assert_parse_equal(rule: Rule, value: &str, expected: &str) {
    let res = LangParser::parse(rule, value);
    if (res.is_err()) {
        panic!("{:?}", res);
    }
    assert!(res.is_ok());

    let returned_value = res.unwrap();
    assert_eq!(expected, returned_value.as_str());
}
