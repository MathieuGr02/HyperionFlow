use hyperion_value::Value;

use crate::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    ast::ASTError,
    expression::{BinExpOp, UnExpOp},
};

/// The AlgebraFolder trait.
/// Allows for the folding of the algebraic definitions defined in [`Algebra`] over a tree structure.
/// The fold occurs from bottom up, where the function receives its children in the tree as input.
pub trait AlgebraFolder {
    type Output;
    type Error: From<ASTError>;

    /// Fold of the [`Algebra::Scan`] type.
    /// The [`StoreScan`] is a leaf node algebra definition and therefor has no input
    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Algebra::Print`] type.
    /// The [`StorePrint`] is a root node algebra definition
    fn fold_print(
        &mut self,
        print: &StorePrint,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Algebra::Project`] type.
    fn fold_project(
        &mut self,
        project: &Project,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Algebra::Filter`] type.
    fn fold_filter(
        &mut self,
        filter: &Filter,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Algebra::Window`] type.
    fn fold_window(
        &mut self,
        window: &Window,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;
}

/// The ExpressionFolder trait.
/// Allows for the folding of the expression definitions defined in [`Expression`] over a tree structure.
/// The fold occurs from bottom up, where the function receives its children in the tree as input.
pub trait ExpressionFolder {
    type Output;
    type Error: From<ASTError>;

    /// Fold of the [`Expression:Escape`] type.
    /// The `Escape` is a leaf node expression
    fn fold_escape(&mut self, escape: &String) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Expression::Literal`] type.
    /// The `Literal` is a leaf node expression
    fn fold_literal(&mut self, literal: &Value) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Expression::Target`] type.
    /// The `Target` is a leaf node expression
    fn fold_target(&mut self, target: &String) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Expression::Unary`] type.
    fn fold_unary_expression(
        &mut self,
        op: &UnExpOp,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error>;

    /// Fold of the [`Expression::Binary`] type.
    fn fold_binary_expression(
        &mut self,
        op: &BinExpOp,
        input_left: Self::Output,
        input_right: Self::Output,
    ) -> Result<Self::Output, Self::Error>;
}
