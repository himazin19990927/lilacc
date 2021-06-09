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
