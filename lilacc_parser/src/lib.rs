use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lilac);

#[cfg(test)]
mod tests {
    use super::*;
    use lilacc_ast::{expr::*, lit::*, op::*, stmt::Stmt};
    use lilacc_lexer::Lexer;

    macro_rules! test_expr {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = lilac::ExprParser::new().parse(lexer).unwrap();

            assert_eq!($expected, result);
        };
    }

    macro_rules! test_stmt {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = lilac::StmtParser::new().parse(lexer).unwrap();

            assert_eq!($expected, result);
        };
    }

    macro_rules! expr_int {
        ($value: expr) => {
            Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: $value.to_string(),
                }),
            })
        };
    }

    macro_rules! expr_ident {
        ($value: expr) => {
            Expr::Ident(ExprIdent {
                name: $value.to_string(),
            })
        };
    }

    macro_rules! expr_binary {
        ($left: expr, $op: expr, $right: expr) => {
            Expr::Binary(ExprBinary {
                left: Box::new($left),
                op: $op,
                right: Box::new($right),
            })
        };
    }

    macro_rules! expr_unary {
        ($op: expr, $expr: expr) => {
            Expr::Unary(ExprUnary {
                op: $op,
                expr: Box::new($expr),
            })
        };
    }

    #[test]
    fn test_primary() {
        test_expr!("10", expr_int!(10));
        test_expr!("a", expr_ident!("a"));
        test_expr!("hoge", expr_ident!("hoge"));

        test_expr!("(10)", expr_int!(10));
        test_expr!("(a)", expr_ident!("a"));
        test_expr!("(hoge)", expr_ident!("hoge"));
    }

    #[test]
    fn test_unary() {
        test_expr!("-10", expr_unary!(UnOp::Neg, expr_int!(10)));
        test_expr!("-x", expr_unary!(UnOp::Neg, expr_ident!("x")));
    }

    #[test]
    fn test_binary() {
        test_expr!("1+2", expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2)));
        test_expr!("1-2", expr_binary!(expr_int!(1), BinOp::Sub, expr_int!(2)));
        test_expr!("1*2", expr_binary!(expr_int!(1), BinOp::Mul, expr_int!(2)));
        test_expr!("1/2", expr_binary!(expr_int!(1), BinOp::Div, expr_int!(2)));

        test_expr!(
            "1+2+3",
            expr_binary!(
                expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2)),
                BinOp::Add,
                expr_int!(3)
            )
        );

        test_expr!(
            "1+2*3",
            expr_binary!(
                expr_int!(1),
                BinOp::Add,
                expr_binary!(expr_int!(2), BinOp::Mul, expr_int!(3))
            )
        );

        test_expr!(
            "(1+2)*3",
            expr_binary!(
                expr_binary!(expr_int!(1), BinOp::Add, expr_int!(2)),
                BinOp::Mul,
                expr_int!(3)
            )
        );
    }

    #[test]
    fn test_binary_relational() {
        test_expr!(
            "a<b",
            expr_binary!(expr_ident!("a"), BinOp::Lt, expr_ident!("b"))
        );

        test_expr!(
            "a<=b",
            expr_binary!(expr_ident!("a"), BinOp::Le, expr_ident!("b"))
        );

        test_expr!(
            "a==b",
            expr_binary!(expr_ident!("a"), BinOp::Eq, expr_ident!("b"))
        );

        test_expr!(
            "a!=b",
            expr_binary!(expr_ident!("a"), BinOp::Ne, expr_ident!("b"))
        );

        test_expr!(
            "a>b",
            expr_binary!(expr_ident!("b"), BinOp::Lt, expr_ident!("a"))
        );

        test_expr!(
            "a>=b",
            expr_binary!(expr_ident!("b"), BinOp::Le, expr_ident!("a"))
        );
    }

    #[test]
    fn test_binary_logical() {
        test_expr!(
            "a&&b",
            expr_binary!(expr_ident!("a"), BinOp::And, expr_ident!("b"))
        );

        test_expr!(
            "a||b",
            expr_binary!(expr_ident!("a"), BinOp::Or, expr_ident!("b"))
        );

        test_expr!(
            "a&&b||c",
            expr_binary!(
                expr_binary!(expr_ident!("a"), BinOp::And, expr_ident!("b")),
                BinOp::Or,
                expr_ident!("c")
            )
        );

        test_expr!(
            "a||b&&c",
            expr_binary!(
                expr_ident!("a"),
                BinOp::Or,
                expr_binary!(expr_ident!("b"), BinOp::And, expr_ident!("c"))
            )
        );
    }

    #[test]
    fn test_stmt() {
        test_stmt!(
            "a+b",
            Stmt::Expr(expr_binary!(expr_ident!("a"), BinOp::Add, expr_ident!("b")))
        );

        test_stmt!(
            "a+b;",
            Stmt::Semi(expr_binary!(expr_ident!("a"), BinOp::Add, expr_ident!("b")))
        );
    }
}
