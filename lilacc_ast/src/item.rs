use crate::pat::*;

pub struct FnSignature {
    pub ident: String,
    pub input: Vec<FnArg>,
}

pub enum FnArg {
    /// A function argument of type-annotated: `x: i32`, `foo: f64`
    Typed(PatType),
}
