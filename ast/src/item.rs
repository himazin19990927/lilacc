use crate::{block::*, pat::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Item {
    /// A function item: `fn foo(x: i32) { ... }`
    Fn(ItemFn),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ItemFn {
    pub sig: FnSignature,
    pub block: Box<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FnSignature {
    pub ident: String,
    pub input: Vec<FnArg>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FnArg {
    /// A function argument of type-annotated: `x: i32`, `foo: f64`
    Typed(PatType),
}
