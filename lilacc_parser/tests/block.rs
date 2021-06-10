use lilacc_ast::{block::*, expr::*, lit::*, op::*, pat::*, stmt::*, *};
use lilacc_lexer::Lexer;
use lilacc_parser::*;

macro_rules! test_block {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::BlockParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_block() {
    test_block!(
        "{let x = 0;}",
        block![Stmt::Local(Local {
            pat: pat_ident!("x"),
            init: expr_int!(0),
        })]
    );

    test_block!(
        "{x+y;}",
        block![Stmt::Semi(expr_binary!(
            expr_ident!("x"),
            BinOp::Add,
            expr_ident!("y")
        ))]
    );

    test_block!(
        "{x+y; a+b;}",
        block![
            Stmt::Semi(expr_binary!(expr_ident!("x"), BinOp::Add, expr_ident!("y"))),
            Stmt::Semi(expr_binary!(expr_ident!("a"), BinOp::Add, expr_ident!("b")))
        ]
    );
}
