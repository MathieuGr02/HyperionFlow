#[cfg(test)]
mod ast_parse {
    use hyperion_lang::ast::AST;

    #[test]
    fn test() {
        let s = "FROM SQL(users) PROJECT SQL(name AS user) FILTER SQL(age > 25) INTO SQL(people)";
        dbg!(&s);
        let ast = AST::parse(s);
        if let Err(e) = ast {
            panic!("{}", e);
        }
    }
}
