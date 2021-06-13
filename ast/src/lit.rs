#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    /// An integer literal: `0`, `1`, `64`
    Int(LitInt),

    /// A bool literal: `true`, `false`
    Bool(LitBool),
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitInt {
    pub digits: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LitBool {
    pub value: bool,
}
