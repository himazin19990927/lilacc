use crate::{block::*, lit::Lit, op::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    /// A binary operation: `a + b`, "a && b"
    Binary(ExprBinary),

    /// A unary operation: `-x`
    Unary(ExprUnary),

    /// A blocked scope: `{ ... }`
    Block(ExprBlock),

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
pub struct ExprBlock {
    pub block: Box<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprLit {
    pub lit: Lit,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprIdent {
    pub name: String,
}
