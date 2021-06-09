use crate::stmt::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}
