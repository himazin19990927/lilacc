use crate::lit::*;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Lit(Lit),

    Ident(String),

    EqEq,
    AndAnd,
    OrOr,

    Plus,
    Minus,
    Star,
    Slash,

    OpenParen,
    CloseParen,
}
