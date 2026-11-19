#[cfg(test)]
mod expression_folder {
    use hyperion_lang::ast::ExpressionAST;
    use hyperion_sql::compiler::SQLExpressionCompiler;

    #[test]
    fn test() {
        let expr_ast = ExpressionAST::parse("SQL(age > 25)").unwrap();
        let mut sql_expr_compiler = SQLExpressionCompiler {};
        let expr = expr_ast.fold(&mut sql_expr_compiler).unwrap();

        assert_eq!(expr.inner, "age > 25")
    }
}
