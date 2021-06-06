use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub lilac);

#[cfg(test)]
mod tests {
    use super::*;
    use lilacc_ast::{lit::*, *};
    use lilacc_lexer::Lexer;

    macro_rules! test_expr {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = lilac::ExprParser::new().parse(lexer).unwrap();

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

        test_expr!("(10)", expr_int!(10));
    }

    #[test]
    fn test_unary() {
        test_expr!("-10", expr_unary!(UnOp::Neg, expr_int!(10)));
    }
}
