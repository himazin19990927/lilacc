use crate::{block::*, pat::*};

pub enum Item {
    /// A function item: `fn foo(x: i32) { ... }`
    Fn(ItemFn),
}

pub struct ItemFn {
    pub sig: FnSignature,
    pub block: Box<Block>,
}

pub struct FnSignature {
    pub ident: String,
    pub input: Vec<FnArg>,
}

pub enum FnArg {
    /// A function argument of type-annotated: `x: i32`, `foo: f64`
    Typed(PatType),
}
