#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Ident(TypeIdent),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeIdent {
    pub name: String,
}
