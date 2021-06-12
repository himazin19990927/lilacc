#[macro_export]
macro_rules! expr_int {
    ($value: expr) => {
        Expr::Lit(ExprLit {
            lit: Lit::Int(LitInt {
                digits: $value.to_string(),
            }),
        })
    };
}

#[macro_export]
macro_rules! expr_ident {
    ($value: expr) => {
        Expr::Ident(ExprIdent {
            name: $value.to_string(),
        })
    };
}

#[macro_export]
macro_rules! expr_binary {
    ($left: expr, $op: expr, $right: expr) => {
        Expr::Binary(ExprBinary {
            left: Box::new($left),
            op: $op,
            right: Box::new($right),
        })
    };
}

#[macro_export]
macro_rules! expr_unary {
    ($op: expr, $expr: expr) => {
        Expr::Unary(ExprUnary {
            op: $op,
            expr: Box::new($expr),
        })
    };
}

#[macro_export]
macro_rules! stmt_local {
    ($pat: expr, $init: expr) => {
        Stmt::Local(Local {
            pat: $pat,
            init: $init,
        })
    };
}

#[macro_export]
macro_rules! stmt_semi {
    ($expr: expr) => {
        Stmt::Semi($expr)
    };
}

#[macro_export]
macro_rules! block {
    ( $( $stmt: expr),* ) => {
        {
            let mut stmts = Vec::new();
            $(
                stmts.push($stmt);
            )*

            Block {
                stmts: stmts,
            }
        }
    }
}

#[macro_export]
macro_rules! type_ident {
    ($ident: expr) => {
        Type::Ident(TypeIdent {
            name: $ident.to_string(),
        })
    };
}

#[macro_export]
macro_rules! pat_ident {
    ($ident: expr) => {
        Pat::Ident(PatIdent {
            ident: $ident.to_string(),
        })
    };
}

#[macro_export]
macro_rules! pat_type {
    ($pat: expr, $ty: expr) => {
        Pat::Type(PatType {
            pat: Box::new($pat),
            ty: Box::new($ty),
        })
    };
}

#[cfg(test)]
mod tests {
    use crate::{block::*, expr::*, lit::*, op::*, pat::*, stmt::*, ty::*};

    #[test]
    fn test_expr_int() {
        assert_eq!(
            Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "10".to_string()
                })
            }),
            expr_int!(10)
        );
    }

    #[test]
    fn test_expr_ident() {
        assert_eq!(
            Expr::Ident(ExprIdent {
                name: "x".to_string()
            }),
            expr_ident!("x")
        );
    }

    #[test]
    fn test_expr_binary() {
        let left = Expr::Lit(ExprLit {
            lit: Lit::Int(LitInt {
                digits: "1".to_string(),
            }),
        });
        let right = Expr::Lit(ExprLit {
            lit: Lit::Int(LitInt {
                digits: "2".to_string(),
            }),
        });

        assert_eq!(
            Expr::Binary(ExprBinary {
                left: Box::new(left),
                op: BinOp::Add,
                right: Box::new(right),
            }),
            expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2))
        )
    }

    #[test]
    fn test_expr_unary() {
        let expr = Expr::Lit(ExprLit {
            lit: Lit::Int(LitInt {
                digits: "10".to_string(),
            }),
        });

        assert_eq!(
            Expr::Unary(ExprUnary {
                op: UnOp::Neg,
                expr: Box::new(expr),
            }),
            expr_unary!(UnOp::Neg, expr_int!(10))
        );
    }

    #[test]
    fn test_stmt_local() {
        let stmt = Stmt::Local(Local {
            pat: Pat::Ident(PatIdent {
                ident: "x".to_string(),
            }),
            init: Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "1".to_string(),
                }),
            }),
        });

        assert_eq!(stmt, stmt_local!(pat_ident!("x"), expr_int!(1)));
    }

    #[test]
    fn test_stmt_semi() {
        let stmt = {
            let left = Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "1".to_string(),
                }),
            });
            let right = Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "2".to_string(),
                }),
            });

            let expr = Expr::Binary(ExprBinary {
                left: Box::new(left),
                op: BinOp::Add,
                right: Box::new(right),
            });

            Stmt::Semi(expr)
        };

        assert_eq!(
            stmt,
            stmt_semi!(expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2)))
        );
    }

    #[test]
    fn test_block() {
        let stmt1 = Stmt::Local(Local {
            pat: Pat::Ident(PatIdent {
                ident: "x".to_string(),
            }),
            init: Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "1".to_string(),
                }),
            }),
        });

        let stmt2 = {
            let left = Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "1".to_string(),
                }),
            });
            let right = Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "2".to_string(),
                }),
            });

            let expr = Expr::Binary(ExprBinary {
                left: Box::new(left),
                op: BinOp::Add,
                right: Box::new(right),
            });

            Stmt::Semi(expr)
        };

        let block = Block {
            stmts: vec![stmt1, stmt2],
        };

        assert_eq!(
            block,
            block![
                Stmt::Local(Local {
                    pat: pat_ident!("x"),
                    init: expr_int!(1)
                }),
                Stmt::Semi(expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2)))
            ]
        );
    }

    #[test]
    fn test_type_ident() {
        assert_eq!(
            Type::Ident(TypeIdent {
                name: "foo".to_string()
            }),
            type_ident!("foo")
        );
    }

    #[test]
    fn test_pat_ident() {
        assert_eq!(
            Pat::Ident(PatIdent {
                ident: "foo".to_string(),
            }),
            pat_ident!("foo")
        );
    }

    #[test]
    fn test_pat_type() {
        assert_eq!(
            Pat::Type(PatType {
                pat: Box::new(Pat::Ident(PatIdent {
                    ident: "x".to_string(),
                })),
                ty: Box::new(Type::Ident(TypeIdent {
                    name: "i32".to_string()
                }))
            }),
            pat_type!(pat_ident!("x"), type_ident!("i32"))
        );
    }
}
