use ast::{block::*, expr::*, item::*, lit::*, op::*, pat::*, stmt::*, ty::*, *};
use lexer::Lexer;
use parser::*;

macro_rules! test_item {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::ItemParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_fn_item() {
    let item1 = {
        let sig = FnSignature {
            ident: "foo".to_string(),
            input: vec![FnArg::Typed(PatType {
                pat: Box::new(pat_ident!("x")),
                ty: Box::new(type_ident!("i32")),
            })],
        };
        let block = block![stmt_semi!(expr_binary!(
            expr_ident!("x"),
            BinOp::Add,
            expr_ident!("y")
        ))];

        let item_fn = ItemFn {
            sig: sig,
            block: Box::new(block),
        };

        Item::Fn(item_fn)
    };
    test_item!("fn foo(x: i32) {x + y;}", item1);

    let item2 = {
        let sig = FnSignature {
            ident: "a".to_string(),
            input: vec![
                FnArg::Typed(PatType {
                    pat: Box::new(pat_ident!("x1")),
                    ty: Box::new(type_ident!("f64")),
                }),
                FnArg::Typed(PatType {
                    pat: Box::new(pat_ident!("x2")),
                    ty: Box::new(type_ident!("f64")),
                }),
            ],
        };

        let block = block![
            stmt_local!(pat_ident!("v"), expr_int!(0)),
            stmt_semi!(expr_binary!(expr_ident!("x"), BinOp::Add, expr_ident!("y")))
        ];

        let item_fn = ItemFn {
            sig: sig,
            block: Box::new(block),
        };

        Item::Fn(item_fn)
    };

    test_item!("fn a(x1: f64, x2: f64) {let v = 0; x + y;}", item2);
}
