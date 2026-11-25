#[cfg(test)]
mod query_compiler {
    use hyperion_lang::ast::LogicalPlan;
    use hyperion_mql::compiler::MQLQueryCompiler;
    use mongodb::bson::doc;

    #[test]
    pub fn test() {
        let res = LogicalPlan::parse("FROM MQL(person) PROJECT age INTO MQL(people)");

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
        assert_eq!(aggregate, vec![doc! {}])
    }
}
