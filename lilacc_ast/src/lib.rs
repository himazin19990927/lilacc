pub mod lit;
pub mod token;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinOp {
    Mul,
    Div,

    Add,
    Sub,

    Lt,
    Le,

    Eq,
    Ne,

    And,
    Or,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnOp {
    Neg,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Binary(ExprBinary),
    Unary(ExprUnary),
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
