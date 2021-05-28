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