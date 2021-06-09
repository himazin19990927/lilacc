use crate::lit::*;
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// A literal token: `1`, `true`
    Lit(Lit),

    /// An identity token: `x`, `foo`
    Ident(String),

    /// The `=` token
    Eq,

    /// The `<` token
    Lt,

    /// The `<=` token
    Le,

    /// The `==` token
    EqEq,

    /// The `!=` token
    Ne,

    /// The `>=` token
    Ge,

    /// The `>` token
    Gt,

    /// The `&&` token
    AndAnd,

    /// The `||` token
    OrOr,

    /// The `!` token
    Not,

    /// The `+` token
    Plus,

    /// The `-` token
    Minus,

    /// The `*` token
    Star,

    /// The `/` token
    Slash,

    /// The `&` token
    And,

    /// The `|` token
    Or,

    /// The `(` token
    OpenParen,

    /// The `)` token
    CloseParen,

    /// The `;` token
    Semi,

    /// The `:` token
    Colon,

    /// The `let` token
    Let,
}
