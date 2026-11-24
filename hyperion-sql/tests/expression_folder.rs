#[cfg(test)]
mod expression_folder {
    use hyperion_lang::ast::LogicalPlan;
    use hyperion_sql::{compiler::SQLQueryCompiler, stores::postgres::database::PostgreSQL};
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

        let s = query.build(&PostgreSQL);
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

        let s = query.build(&PostgreSQL);
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

        let s = query.build(&PostgreSQL);
        assert_eq!(s, format!("SELECT age FROM users WHERE {filter}"));
    }

    #[test]
    fn query_tumble_window() {
        let query = format!(
            "FROM SQL(users) PROJECT age FILTER EXP(age) TUMBLE_WINDOW(col=_processing_time, size=10) INTO SQL(people)"
        );

        let ast = LogicalPlan::parse(&query);
        if let Err(e) = ast {
            panic!("{}", e)
        }

        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }

        let query = query.unwrap();

        let s = query.build(&PostgreSQL);
        assert_eq!(
            s,
            format!(
                "SELECT age FROM users WHERE (EXP(age)) AND (_processing_time >= $1 AND _processing_time <= $2)"
            )
        )
    }

    #[test]
    fn query_count_window() {
        let query = format!(
            "FROM SQL(users) PROJECT age FILTER EXP(age) COUNT_WINDOW(size=10) INTO SQL(people)"
        );

        let ast = LogicalPlan::parse(&query);
        if let Err(e) = ast {
            panic!("{}", e)
        }

        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{}", e)
        }

        let query = query.unwrap();

        let s = query.build(&PostgreSQL);
        assert_eq!(s, format!("SELECT age FROM users WHERE EXP(age)"))
    }
}
