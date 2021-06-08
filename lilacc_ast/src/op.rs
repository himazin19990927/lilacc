#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    /// The `*` operator (multiplication)
    Mul,

    /// The `/` operator (division)
    Div,

    /// The `+` operator (addition)
    Add,

    /// The `-` operator (subtraction)
    Sub,

    /// The `<` operator (less than)
    Lt,

    /// The `<=` operator (less than or equal to)
    Le,

    /// The `==` operator (equal to)
    Eq,

    /// The `!=` operator (not equal to)
    Ne,

    /// The `&&` operator (logical and)
    And,

    /// The `||` operator (logical or)
    Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    /// The `-` operator (negation)
    Neg,
}