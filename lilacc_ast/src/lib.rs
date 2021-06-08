pub mod lit;
pub mod token;

use lit::Lit;

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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a && b"
    Binary(ExprBinary),

    /// A unary operation: `-x`
    Unary(ExprUnary),

    /// A literal in place of an expression: `1`, `true`
    Lit(ExprLit),

    /// An identity like `x`, `foo`
    Ident(ExprIdent),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprIdent {
    pub name: String,
}
