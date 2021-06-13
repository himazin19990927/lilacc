use crate::{expr::*, pat::Pat};

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// A local binding: `let x;`, `let v = 10;`
    Local(Local),

    /// An expression with `;`: `a + b;`, `a = 10;`
    Semi(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Local {
    pub pat: Pat,
    pub init: Expr,
}
