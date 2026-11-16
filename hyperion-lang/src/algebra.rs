pub enum Algebra {
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
