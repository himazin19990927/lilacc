use lilacc_ast::{expr::*, lit::*, op::*, pat::*, stmt::*, ty::*, *};
use lilacc_lexer::Lexer;
use lilacc_parser::*;

macro_rules! test_stmt {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::StmtParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_local() {
    test_stmt!(
        "let x = 0;",
        Stmt::Local(Local {
            pat: pat_ident!("x"),
            init: expr_int!(0)
        })
    );

    test_stmt!(
        "let x: i32 = 0;",
        Stmt::Local(Local {
            pat: pat_type!(pat_ident!("x"), type_ident!("i32")),
            init: expr_int!(0)
        })
    );
}

#[test]
fn parse_expr() {
    test_stmt!(
        "a+b",
        Stmt::Expr(expr_binary!(expr_ident!("a"), BinOp::Add, expr_ident!("b")))
    );
}

#[test]
fn parse_semi() {
    test_stmt!(
        "a+b;",
        Stmt::Semi(expr_binary!(expr_ident!("a"), BinOp::Add, expr_ident!("b")))
    );
}
