#[cfg(test)]
mod expression_folder {
    use hyperion_lang::ast::{AlgebraAST, ExpressionAST};
    use hyperion_sql::compiler::{SQLExpressionCompiler, SQLQueryCompiler};
    use rstest::rstest;
    /*
    #[test]
    fn test() {
        let expr_ast = ExpressionAST::parse("SQL(age > 25)").unwrap();
        let mut sql_expr_compiler = SQLExpressionCompiler {};
        let expr = expr_ast.fold(&mut sql_expr_compiler).unwrap();

        assert_eq!(expr.inner, "age > 25")
    }
    */
    /*
    #[rstest]
    #[case("ABS")]
    #[case("AVG")]
    #[case("CEILING")]
    #[case("COUNT")]
    #[case("EXP")]
    #[case("FLOOR")]
    #[case("MAX")]
    #[case("MIN")]
    #[case("SIGN")]
    #[case("SQRT")]
    #[case("SQUARE")]
    #[case("SUM")]
    fn query_unary_function(#[case] f: &str) {
        let query = format!("FROM SQL(users) PROJECT {f}(age) INTO SQL(people)");
        let ast = AlgebraAST::parse(query.as_str());
        if let Err(e) = ast {
            panic!("{}", e)
        }
        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }
        let query = query.unwrap();

        let s = query.build();
        assert_eq!(s, format!("SELECT {f}(age) FROM users"))
    }

    #[rstest]
    #[case("age")]
    #[case("name, age")]
    #[case("name, age, country")]
    fn query_projection(#[case] columns: &str) {
        let query = format!("FROM SQL(users) PROJECT {columns} INTO SQL(people)");
        let ast = AlgebraAST::parse(query.as_str());
        if let Err(e) = ast {
            panic!("{}", e)
        }
        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }
        let query = query.unwrap();

        let s = query.build();
        assert_eq!(s, format!("SELECT {columns} FROM users"))
    }
    */
    #[test]
    fn query_filter() {
        let query =
            format!("FROM SQL(users) PROJECT age FILTER EXP(EXP(EXP(age))) INTO SQL(people)");
        let ast = AlgebraAST::parse(query.as_str());
        if let Err(e) = ast {
            panic!("{}", e)
        }
        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }
        let query = query.unwrap();

        let s = query.build();
        assert_eq!(s, "SELECT age FROM users WHERE age > 25");
    }
}
