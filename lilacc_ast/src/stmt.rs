use crate::expr::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    /// A local binding: `let x;`, `let v = 10;`
    Local(Local),

    /// An expression without `;`: `a + b`, `true`
    Expr(Expr),

    /// An expression with `;`: `a + b;`, `a = 10;`
    Semi(Expr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Local {
    pub ident: ExprIdent,
    pub init: Expr,
}
