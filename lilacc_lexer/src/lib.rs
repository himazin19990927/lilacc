use core::panic;
use lilacc_token::{Token, TokenKind};
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

        let mut lit = String::from(self.ch.unwrap());
        loop {
            self.read_char();
            if let Some(c) = self.ch {
                if c.is_digit(10) {
                    lit.push(c);
                    continue;
                }
            }
            break;
        }

        Token {
            kind: TokenKind::Num,
            literal: lit,
        }
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
                '+' => Token::new(TokenKind::Plus, ch),
                '-' => Token::new(TokenKind::Minus, ch),
                '*' => Token::new(TokenKind::Star, ch),
                '/' => Token::new(TokenKind::Slash, ch),

                '(' => Token::new(TokenKind::OpenParen, ch),
                ')' => Token::new(TokenKind::CloseParen, ch),

                '0'..='9' => return Some(self.read_number()),
                _ => {
                    let literal = self.read_str();
                    return Some(Token::new(TokenKind::Ident, literal));
                }
            }),
            None => None,
        };

        self.read_char();

        token
    }
}

#[cfg(test)]
mod tests {
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
        test_lexer!("1", vec![Token::new(TokenKind::Num, "1"),]);
        test_lexer!("10", vec![Token::new(TokenKind::Num, "10"),]);

        test_lexer!(
            "10+20",
            vec![
                Token::new(TokenKind::Num, "10"),
                Token::new(TokenKind::Plus, "+"),
                Token::new(TokenKind::Num, "20"),
            ]
        );
    }

    #[test]
    fn paren() {
        test_lexer!(
            "()",
            vec![
                Token::new(TokenKind::OpenParen, "("),
                Token::new(TokenKind::CloseParen, ")"),
            ]
        );

        test_lexer!(
            "(1)",
            vec![
                Token::new(TokenKind::OpenParen, "("),
                Token::new(TokenKind::Num, "1"),
                Token::new(TokenKind::CloseParen, ")"),
            ]
        );
    }

    #[test]
    fn ident() {
        test_lexer!("a", vec![Token::new(TokenKind::Ident, "a")]);
        test_lexer!("ab", vec![Token::new(TokenKind::Ident, "ab")]);
        test_lexer!(
            "a1+a2",
            vec![
                Token::new(TokenKind::Ident, "a1"),
                Token::new(TokenKind::Plus, "+"),
                Token::new(TokenKind::Ident, "a2")
            ]
        );
    }
}
