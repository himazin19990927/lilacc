use ast::{expr::*, lit::*, op::*, *};
use lexer::Lexer;
use parser::*;

macro_rules! test_expr {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::ExprParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_primary() {
    test_expr!("10", expr_int!(10));
    test_expr!("a", expr_ident!("a"));
    test_expr!("hoge", expr_ident!("hoge"));

    test_expr!("(10)", expr_int!(10));
    test_expr!("(a)", expr_ident!("a"));
    test_expr!("(hoge)", expr_ident!("hoge"));
}

#[test]
fn parse_unary() {
    test_expr!("-10", expr_unary!(UnOp::Neg, expr_int!(10)));
    test_expr!("-x", expr_unary!(UnOp::Neg, expr_ident!("x")));
}

#[test]
fn parse_binary() {
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
fn parse_binary_relational() {
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
fn parse_binary_logical() {
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
