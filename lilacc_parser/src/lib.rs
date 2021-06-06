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

    #[test]
    fn test_primary() {
        test_expr!(
            "10",
            Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "10".to_string(),
                }),
            })
        );

        test_expr!(
            "(10)",
            Expr::Lit(ExprLit {
                lit: Lit::Int(LitInt {
                    digits: "10".to_string(),
                }),
            })
        );
    }

    #[test]
    fn test_unary() {
        test_expr!(
            "-10",
            Expr::Unary(ExprUnary {
                op: UnOp::Neg,
                expr: Box::new(Expr::Lit(ExprLit {
                    lit: Lit::Int(LitInt {
                        digits: "10".to_string(),
                    }),
                }))
            })
        );
    }
}
