use lilacc_ast::{ty::*, *};
use lilacc_lexer::Lexer;
use lilacc_parser::*;

macro_rules! test_type {
    ($input: expr, $expected: expr) => {
        let lexer = Lexer::new($input);
        let result = lilac::TypeParser::new().parse(lexer).unwrap();

        assert_eq!($expected, result);
    };
}

#[test]
fn parse_ident() {
    test_type!("i32", type_ident!("i32"));
}
