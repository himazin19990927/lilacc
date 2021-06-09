use crate::ty::Type;

#[derive(Debug, PartialEq, Clone)]
pub enum Pat {
    /// A pattern that binds a new variable: `x`, `foo`
    Ident(PatIdent),

    /// A pattern with type annotation: `x: i32`, `foo: f64`
    Type(PatType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatIdent {
    pub ident: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PatType {
    pub pat: Box<Pat>,
    pub ty: Box<Type>,
}
