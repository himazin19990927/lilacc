use ast::{pat::*, ty::*, *};
use lexer::Lexer;
use parser::*;

macro_rules! test_pat {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::PatParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_ident() {
    test_pat!("foo", pat_ident!("foo"));
}

#[test]
fn parse_type() {
    test_pat!("x:i32", pat_type!(pat_ident!("x"), type_ident!("i32")));
}
