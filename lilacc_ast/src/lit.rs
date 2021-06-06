#[derive(Debug, PartialEq, Clone)]
pub enum Lit {
    Int(LitInt),
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
