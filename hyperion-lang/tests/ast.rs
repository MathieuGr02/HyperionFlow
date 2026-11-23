#[cfg(test)]
mod ast_parse {
    use hyperion_lang::ast::LogicalPlan;

    #[test]
    fn test() {
        let s = "FROM SQL(users) PROJECT SQL(name AS user) FILTER SQL(age > 25) COUNT_WINDOW(size=10) INTO SQL(people)";
        dbg!(&s);
        let ast = LogicalPlan::parse(s);
        if let Err(e) = ast {
            panic!("{}", e);
        }
    }
}
