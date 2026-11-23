#[cfg(test)]
mod expression_folder {
    use hyperion_lang::ast::{LogicalExpression, LogicalPlan};
    use hyperion_sql::compiler::{SQLExpressionCompiler, SQLQueryCompiler};
    use rstest::rstest;

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
        let ast = LogicalPlan::parse(query.as_str());
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
    #[case("name, FLOOR(age), country")]
    fn query_projection(#[case] columns: &str) {
        let query = format!("FROM SQL(users) PROJECT {columns} INTO SQL(people)");
        let ast = LogicalPlan::parse(query.as_str());
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

    #[rstest]
    #[case("EXP(age)")]
    #[case("EXP(EXP(age))")]
    fn query_filter(#[case] filter: &str) {
        let query = format!("FROM SQL(users) PROJECT age FILTER {filter} INTO SQL(people)");
        let ast = LogicalPlan::parse(query.as_str());
        if let Err(e) = ast {
            panic!("{}", e)
        }
        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }
        let query = query.unwrap();

        dbg!(&query);

        let s = query.build();
        assert_eq!(s, format!("SELECT age FROM users WHERE {filter}"));
    }
}
