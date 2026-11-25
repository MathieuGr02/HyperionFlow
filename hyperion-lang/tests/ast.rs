#[cfg(test)]
mod ast_parse {
    use hyperion_lang::ast::LogicalPlan;

    #[test]
    fn test() {
        let s = "FROM SQL(users) PROJECT EXP(age) AS age FILTER SQL(age > 25) COUNT_WINDOW(size=10) INTO SQL(people)";
        let ast = LogicalPlan::parse(s);
        dbg!(&ast);
        if let Err(e) = ast {
            panic!("{}", e);
        }
    }
}
