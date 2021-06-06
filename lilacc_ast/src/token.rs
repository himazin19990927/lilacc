use crate::lit::*;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Lit(Lit),

    Ident(String),

    Eq,
    Lt,
    Le,
    EqEq,
    Ne,
    Ge,
    Gt,
    AndAnd,
    OrOr,
    Not,

    Plus,
    Minus,
    Star,
    Slash,
    And,
    Or,

    OpenParen,
    CloseParen,
}
