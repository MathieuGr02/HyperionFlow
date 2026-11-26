use hyperion_lang::{
    algebra::{Filter, Project, StorePrint, StoreScan, Window},
    arena_tree::{ArenaTree, Ref},
    ast::{ASTError, LogicalExpression},
    expression::{BinExpOp, UnExpOp},
    folder::{AlgebraFolder, ExpressionFolder},
};
use hyperion_value::Value;
use mongodb::bson::{self, Binary, Bson, Document, doc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MQLError {
    #[error("{0}")]
    ASTError(#[from] ASTError),
    #[error("{0}")]
    Custom(String),
}

#[derive(Debug)]
pub struct ParameterisedMQLQuery {
    left: Vec<Document>,
    parameterised: Document,
    right: Vec<Document>,
}

impl ParameterisedMQLQuery {
    pub fn set(&self) -> Vec<Document> {
        let resolved_document = Document::new();
        let mut left = self.left.clone();
        left.push(resolved_document);
        left.append(&mut self.right.clone());
        left
    }
}

#[derive(Debug)]
pub struct MQLQuery {
    target: Option<MQLTarget>,
    project: Vec<MQLProject>,
    filter: Option<ParameterisedMQLExpression>,
    source: MQLSource,
}

#[derive(Debug)]
pub struct MQLProject {
    pub field: String,
    pub expr: ParameterisedMQLExpression
}

#[derive(Debug)]
pub struct MQLTarget {
    collection: String,
}

#[derive(Debug)]
pub struct MQLSource {
    collection: String,
}

impl MQLQuery {
    /// Build the MQL query into an aggregate pipeline query.
    /// Returns the source collection and the aggregate pipeline
    pub fn build(&self) -> (String, Vec<Document>) {
        let mut pipeline = Vec::new();

        for p in self.project.iter() {
            let x = p.build();
        }

        if let Some(target) = self.target.as_ref() {
            let collection = &target.collection;
            let d = doc! { "$merge": { "into": collection } };
            pipeline.push(d);
        }

        (self.source.collection.clone(), pipeline)
    }
}

pub struct MQLQueryCompiler;

impl AlgebraFolder for MQLQueryCompiler {
    type Output = MQLQuery;
    type Error = MQLError;

    fn fold_scan(&mut self, scan: &StoreScan) -> Result<Self::Output, Self::Error> {
        match scan {
            StoreScan::MQLScan(mqlscan) => Ok(MQLQuery {
                target: None,
                filter: None,
                project: Vec::new(),
                source: MQLSource {
                    collection: mqlscan.collection.clone(),
                },
            }),
            _ => Err(MQLError::Custom("Invalid scan".to_string())),
        }
    }

    fn fold_print(
        &mut self,
        print: &StorePrint,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        match print {
            StorePrint::MQLPrint(mqlprint) => {
                let target = MQLTarget {
                    collection: mqlprint.collection.clone(),
                };
                input.target = Some(target);
                Ok(input)
            }
            _ => Err(MQLError::Custom("Invalid print".to_string())),
        }
    }

    fn fold_project(
        &mut self,
        project: &Project,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        for expr in project.expression.iter() {
            let param_expr = MQLExpressionCompiler::compile(expr)?;
            param_expr.
            input.project.push(param_expr);
        }
        Ok(input)
    }

    fn fold_filter(
        &mut self,
        filter: &Filter,
        mut input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let expr = MQLExpressionCompiler::compile(&filter.expression)?;
        input.filter = Some(expr);
        Ok(input)
    }

    fn fold_window(
        &mut self,
        window: &Window,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub enum MQLExpression {
    Raw(Document),
    Literal(Bson),
    Identifier(String),
    PlaceHolder(usize),
    Unary { op: String, input: Ref },
    Binary { op: String, left: Ref, right: Ref },
}

/// The Parameterised MQL expression.
/// Holds a MQL compiled expression with references to the [`MQLExpression::PlaceHolder`] fields
#[derive(Debug)]
pub struct ParameterisedMQLExpression {
    tree: ArenaTree<MQLExpression>,
    placeholders: Vec<Ref>,
}

impl ParameterisedMQLExpression {
    pub fn new() -> Self {
        Self {
            tree: ArenaTree::new(),
            placeholders: Vec::new(),
        }
    }

    /// Set the expression at the specific placeholder value
    pub fn set(&mut self, index: usize, expr: MQLExpression) {
        let placeholder_ref = self.placeholders.remove(index);
        self.tree.replace(placeholder_ref, expr);
    }

    /// If the expression has any placeholder values in its definition.
    pub fn has_args(&self) -> bool {
        self.placeholders.len() != 0
    }

    pub fn build(&self) -> Bson {
        if self.has_args() {
            panic!(
                "Cannot build Parameterised MQL expression as it still has placeholder expressions"
            );
        }

        if let Some(root) = self.tree.get_root() {
            self.build_recurse(*root)
        } else {
            panic!("No root for tree");
        }
    }

    fn build_recurse(&self, index: Ref) -> Bson {
        match self.tree.get_unchecked(index) {
            MQLExpression::Raw(document) => Bson::Document(document.to_owned()),
            MQLExpression::Unary { op, input } => {
                let inner = self.build_recurse(*input);
                let key = format!("${op}");
                Bson::Document(doc! { key: inner })
            }
            MQLExpression::Binary { op, left, right } => {
                let inner_left = self.build_recurse(*left);
                let inner_right = self.build_recurse(*right);
                let key = format!("${op}");
                Bson::Document(doc! { key: [ inner_left, inner_right ] })
            }
            MQLExpression::Literal(bson) => bson.to_owned(),
            MQLExpression::Identifier(identifier) => Bson::String(identifier.to_owned()),
            MQLExpression::PlaceHolder(_) => todo!(),
        }
    }
}

/// The MQL expression compiler.
/// Is a stateful compiler holding a [`ParameterisedMQLExpression`], mutating it during the folding of the expression.
pub struct MQLExpressionCompiler {
    expr: ParameterisedMQLExpression,
}

impl MQLExpressionCompiler {
    /// Compile a [`LogicalExpression`] into a [`ParameterisedMQLExpression`]
    pub fn compile(expr: &LogicalExpression) -> Result<ParameterisedMQLExpression, MQLError> {
        let mut compiler = Self {
            expr: ParameterisedMQLExpression::new(),
        };
        let root = expr.fold(&mut compiler)?;
        compiler.expr.tree.set_root(root);

        Ok(compiler.expr)
    }
}

impl ExpressionFolder for MQLExpressionCompiler {
    type Output = Ref;
    type Error = MQLError;

    fn fold_escape(&mut self, escape: &String) -> Result<Self::Output, Self::Error> {
        let doc = bson::to_document(escape).map_err(|e| MQLError::Custom(e.to_string()))?;
        let expr = MQLExpression::Raw(doc);
        let index = self.expr.tree.add(expr);

        Ok(index)
    }

    fn fold_literal(&mut self, literal: &Value) -> Result<Self::Output, Self::Error> {
        let b = match literal {
            Value::Null => Bson::Null,
            Value::Boolean(boolean) => Bson::Boolean(*boolean),
            Value::Bytes(bytes) => Bson::Binary(Binary {
                subtype: bson::spec::BinarySubtype::Generic,
                bytes: bytes.to_owned().as_vec(),
            }),
            Value::Integer(integer) => Bson::Int64(integer.as_i64()),
            Value::Float(float) => Bson::Double(float.as_f64()),
            Value::String(string) => Bson::String(string.to_owned()),
            Value::Timestamp(timestamp) => todo!(),
            Value::Array(values) => todo!(),
            Value::Map(map) => todo!(),
            Value::Node(node) => todo!(),
            Value::Edge(edge) => todo!(),
        };

        let expr = MQLExpression::Literal(b);
        let index = self.expr.tree.add(expr);

        Ok(index)
    }

    fn fold_target(&mut self, target: &String) -> Result<Self::Output, Self::Error> {
        let expr = MQLExpression::Identifier(target.to_owned());
        let index = self.expr.tree.add(expr);

        Ok(index)
    }

    fn fold_unary_expression(
        &mut self,
        op: &UnExpOp,
        input: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let op_str = match op {
            UnExpOp::Abs => "abs",
            UnExpOp::Avg => "avg",
            UnExpOp::Ceiling => "ceil",
            UnExpOp::Count => todo!(),
            UnExpOp::Exp => "exp",
            UnExpOp::Floor => "floor",
            UnExpOp::Max => "max",
            UnExpOp::Min => "min",
            UnExpOp::Sign => todo!(),
            UnExpOp::Sqrt => "sqrt",
            UnExpOp::Square => todo!(),
            UnExpOp::Sum => "sum",
        };

        let expr = MQLExpression::Unary {
            op: op_str.to_string(),
            input,
        };
        let index = self.expr.tree.add(expr);

        Ok(index)
    }

    fn fold_binary_expression(
        &mut self,
        op: &BinExpOp,
        input_left: Self::Output,
        input_right: Self::Output,
    ) -> Result<Self::Output, Self::Error> {
        let op_str = match op {
            BinExpOp::Rename => ,
            BinExpOp::Eq => "eq",
            BinExpOp::Geq => "gte",
            BinExpOp::Leq => "lte",
            BinExpOp::Between => todo!(),
        };

        let expr = MQLExpression::Binary {
            op: op_str.to_string(),
            left: input_left,
            right: input_right,
        };
        let index = self.expr.tree.add(expr);

        Ok(index)
    }
}
