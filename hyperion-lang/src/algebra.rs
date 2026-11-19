/*pub enum Algebra {
    // Common algebra
    Scan,
    Project,
    Filter,
    Join,

    // Relational algebra
    Union,
    Different,
    Intersection,

    // Document algebra
    Unwind,

    // Graph algebra
    Path,
}
*/

use crate::ast::{ExpressionAST, Ref};

///
#[derive(Debug)]
pub enum UnaryAlgebra {
    Filter(Filter),
    Project(Project),
}

#[derive(Debug)]
pub struct Filter {
    pub(crate) expression: ExpressionAST,
}

#[derive(Debug)]
pub struct Project {
    pub(crate) expression: ExpressionAST,
}

#[derive(Debug)]
pub enum BinaryAlgebra {
    Join(Join),
}

#[derive(Debug)]
pub struct Join {}

#[derive(Debug)]
pub enum StoreScan {
    SQLScan(SQLScan),
    MQLScan(MQLScan),
    CypherScan(CypherScan),
}

#[derive(Debug)]
pub enum StorePrint {
    SQLPrint(SQLPrint),
    MQLPrint(MQLPrint),
    CypherPrint(CypherPrint),
}

#[derive(Debug)]
pub struct SQLPrint {
    pub table: String,
}

#[derive(Debug)]
pub struct MQLPrint {
    pub collection: String,
}

#[derive(Debug)]
pub struct CypherPrint {
    pub graph: String,
}

#[derive(Debug)]
pub struct SQLScan {
    pub table: String,
}

#[derive(Debug)]
pub struct MQLScan {
    pub collection: String,
}

#[derive(Debug)]
pub struct CypherScan {
    pub graph: String,
}

#[derive(Debug)]
pub enum Algebra {
    /// Root node algebra.
    /// Represents the "printing" of data to a specific store
    Print {
        print: StorePrint,
        input: Ref,
    },
    /// Leaf node algebra.
    /// Represents the "scanning" of a specific store, i.e. the location of the data
    Scan(StoreScan),
    // Inner node unary algebra
    // Represents the algebra:
    // A
    // |
    // B
    Unary {
        alg: UnaryAlgebra,
        input: Ref,
    },
    // Inner node binary algebra
    // Represents the algebra:
    //  A
    // / \
    // B  C
    Binary {
        alg: BinaryAlgebra,
        left: Ref,
        right: Ref,
    },
}
