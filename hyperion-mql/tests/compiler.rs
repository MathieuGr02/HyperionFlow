#[cfg(test)]
mod query_compiler {
    use hyperion_lang::ast::LogicalPlan;
    use hyperion_mql::compiler::MQLQueryCompiler;
    use mongodb::bson::{Document, doc};
    use rstest::rstest;

    #[rstest]
    #[case(
        "ABS(age) AS age",
        [doc! { "$project": { "age": { "$abs": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "AVG(age) AS age",
        [doc! { "$project": { "age": { "$avg": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "CEILING(age) AS age",
        [doc! { "$project": { "age": { "$ceil": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    //#[case(
    //    "COUNT(age) AS age",
    //    [doc! { "$project": { "age": { "$count": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    //)]
    #[case(
        "EXP(age) AS age",
        [doc! { "$project": { "age": { "$exp": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "FLOOR(age) AS age",
        [doc! { "$project": { "age": { "$floor": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "MAX(age) AS age",
        [doc! { "$project": { "age": { "$max": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "MIN(age) AS age",
        [doc! { "$project": { "age": { "$min": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    //#[case(
    //    "SIGN(age) AS age",
    //    [doc! { "$project": { "age": { "$sign": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    //)]
    #[case(
        "SQRT(age) AS age",
        [doc! { "$project": { "age": { "$sqrt": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    //#[case(
    //    "SQUARE(age) AS age",
    //    [doc! { "$project": { "age": { "$square": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    //)]
    #[case(
        "SUM(age) AS age",
        [doc! { "$project": { "age": { "$sum": "$age" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    pub fn mql_unary_function(#[case] project: &str, #[case] expected: [Document; 2]) {
        let res = LogicalPlan::parse(&format!(
            "FROM MQL(person) PROJECT {project} INTO MQL(people)"
        ));

        if let Err(e) = res {
            panic!("{}", e);
        }

        let plan = res.unwrap();

        let res = plan.fold(&mut MQLQueryCompiler);

        if let Err(e) = res {
            panic!("{}", e)
        }

        let query = res.unwrap();

        let (collection, aggregate) = query.build();

        assert_eq!(collection, "person");
        assert_eq!(aggregate, expected);
    }

    #[rstest]
    #[case(
        "age",
        [doc! { "$project": { "age": 1 } }, doc! { "$merge": { "into": "people" } }] )]
    #[case(
        "age, value",
        [doc! { "$project": { "age": 1, "value": 1 } }, doc! { "$merge": { "into": "people" } }] )]
    #[case(
        "EXP(age) AS exp_age, value",
        [doc! { "$project": { "exp_age": { "$exp": "$age" }, "value": 1 } }, doc! { "$merge": { "into": "people" } }] )]
    #[case(
        "age, EXP(value) AS exp_value",
        [doc! { "$project": { "age": 1, "exp_value": { "$exp": "$value" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    #[case(
        "EXP(age) AS exp_age, EXP(value) AS exp_value",
        [doc! { "$project": { "exp_age": { "$exp": "$age" }, "exp_value": { "$exp": "$value" } } }, doc! { "$merge": { "into": "people" } }]
    )]
    pub fn mql_project(#[case] project: &str, #[case] expected: [Document; 2]) {
        let res = LogicalPlan::parse(&format!(
            "FROM MQL(person) PROJECT {project} INTO MQL(people)"
        ));

        if let Err(e) = res {
            panic!("{}", e);
        }

        let plan = res.unwrap();

        let res = plan.fold(&mut MQLQueryCompiler);

        if let Err(e) = res {
            panic!("{}", e)
        }

        let query = res.unwrap();

        let (collection, aggregate) = query.build();

        assert_eq!(collection, "person");
        assert_eq!(aggregate, expected);
    }
}
