use core::panic;
use lilacc_ast::{lit::*, token::*};
use std::str::Chars;

#[derive(Debug)]
pub struct Lexer<'input> {
    chars: Chars<'input>,
    ch: Option<char>,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        let mut lexer = Lexer {
            chars: input.chars(),
            ch: None,
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        self.ch = self.chars.next();
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if !c.is_whitespace() {
                break;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        match self.ch {
            Some(ch) => {
                if !ch.is_digit(10) {
                    panic!("A non-numeric value was entered")
                }
            }
            None => panic!("Entered string has already reached the end."),
        }

        let mut digits = String::from(self.ch.unwrap());
        loop {
            self.read_char();
            if let Some(c) = self.ch {
                if c.is_digit(10) {
                    digits.push(c);
                    continue;
                }
            }
            break;
        }

        Token::Lit(Lit::Int(LitInt { digits }))
    }

    fn read_str(&mut self) -> String {
        let is_letter = |c: char| c.is_ascii_alphanumeric() || c == '_';

        let ch = self
            .ch
            .expect("Entered string has already reached the end.");

        if !is_letter(ch) {
            panic!("A non-alphanumeric character was enterd.");
        }

        let mut literal = String::from(ch);
        loop {
            self.read_char();
            match self.ch {
                Some(ch) => {
                    if is_letter(ch) {
                        literal.push(ch);
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }

        literal
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            Some(ch) => Some(match ch {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '/' => Token::Slash,

                '(' => Token::OpenParen,
                ')' => Token::CloseParen,

                '0'..='9' => return Some(self.read_number()),
                _ => {
                    let token = match self.read_str().as_str() {
                        "true" => Token::Lit(Lit::Bool(LitBool { value: true })),
                        "false" => Token::Lit(Lit::Bool(LitBool { value: false })),
                        ident => Token::Ident(ident.to_string()),
                    };

                    return Some(token);
                }
            }),
            None => None,
        };

        self.read_char();

        token
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = (Token, (), ());

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Some(token) => Some((token, (), ())),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    macro_rules! test_lexer {
        ($input: expr, $expected: expr) => {
            let mut lexer = Lexer::new($input);
            let mut tokens: Vec<Token> = Vec::new();

            while let Some(token) = lexer.next_token() {
                tokens.push(token);
            }

            assert_eq!($expected, tokens);
        };
    }

    #[test]
    fn num() {
        test_lexer!(
            "1",
            vec![Token::Lit(Lit::Int(LitInt {
                digits: "1".to_string()
            }))]
        );

        test_lexer!(
            "10",
            vec![Token::Lit(Lit::Int(LitInt {
                digits: "10".to_string()
            }))]
        );

        test_lexer!(
            "10+20",
            vec![
                Token::Lit(Lit::Int(LitInt {
                    digits: "10".to_string()
                })),
                Token::Plus,
                Token::Lit(Lit::Int(LitInt {
                    digits: "20".to_string()
                })),
            ]
        );
    }

    #[test]
    fn paren() {
        test_lexer!("()", vec![Token::OpenParen, Token::CloseParen,]);

        test_lexer!(
            "(1)",
            vec![
                Token::OpenParen,
                Token::Lit(Lit::Int(LitInt {
                    digits: "1".to_string()
                })),
                Token::CloseParen,
            ]
        );
    }

    #[test]
    fn ident() {
        test_lexer!("a", vec![Token::Ident("a".to_string())]);

        test_lexer!("ab", vec![Token::Ident("ab".to_string())]);

        test_lexer!(
            "a1+a2",
            vec![
                Token::Ident("a1".to_string()),
                Token::Plus,
                Token::Ident("a2".to_string())
            ]
        );
    }

    #[test]
    fn lit_bool() {
        test_lexer!("true", vec![Token::Lit(Lit::Bool(LitBool { value: true }))]);
        test_lexer!(
            "false",
            vec![Token::Lit(Lit::Bool(LitBool { value: false }))]
        );

        test_lexer!(
            "(true)",
            vec![
                Token::OpenParen,
                Token::Lit(Lit::Bool(LitBool { value: true })),
                Token::CloseParen,
            ]
        );
    }
}
