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
        let query = format!("FROM SQL(users) PROJECT {f}(age) AS age INTO SQL(people)");
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

        if let Err(e) = s {
            panic!("{e}")
        }

        let s = s.unwrap();

        assert_eq!(
            s,
            format!("INSERT INTO people (age) SELECT {f}(age) AS age FROM users")
        )
    }

    #[rstest]
    #[case("age", "INSERT INTO people (age) SELECT age FROM users")]
    #[case(
        "age, value",
        "INSERT INTO people (age, value) SELECT age, value FROM users"
    )]
    #[case(
        "EXP(age) AS exp_age, value",
        "INSERT INTO people (exp_age, value) SELECT EXP(age) AS exp_age, value FROM users"
    )]
    #[case(
        "age, EXP(value) AS exp_value",
        "INSERT INTO people (age, exp_value) SELECT age, EXP(value) AS exp_value FROM users"
    )]
    #[case(
        "EXP(age) AS exp_age, EXP(value) AS exp_value",
        "INSERT INTO people (exp_age, exp_value) SELECT EXP(age) AS exp_age, EXP(value) AS exp_value FROM users"
    )]
    fn query_projection(#[case] project: &str, #[case] expected: &str) {
        let query = format!("FROM SQL(users) PROJECT {project} INTO SQL(people)");
        let ast = LogicalPlan::parse(query.as_str());
        if let Err(e) = ast {
            panic!("{}", e)
        }
        let ast = ast.unwrap();

        let query = ast.fold(&mut SQLQueryCompiler);
        if let Err(e) = query {
            panic!("{e}")
        }
        let query = query.unwrap();

        let s = query.build(&PostgreSQL);

        if let Err(e) = s {
            panic!("{e}")
        }

        let s = s.unwrap();

        assert_eq!(s, expected)
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

        if let Err(e) = s {
            panic!("{e}")
        }

        let s = s.unwrap();

        assert_eq!(
            s,
            format!("INSERT INTO people (age) SELECT age FROM users WHERE {filter}")
        );
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

        if let Err(e) = s {
            panic!("{e}")
        }

        let s = s.unwrap();

        assert_eq!(
            s,
            format!(
                "INSERT INTO people (age) SELECT age FROM users WHERE (EXP(age)) AND (_processing_time >= $1 AND _processing_time <= $2)"
            )
        )
    }
}
