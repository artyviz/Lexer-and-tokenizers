use std::fmt::{Display, Formatter};

use crate::ast::span::TextSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // literals
    Number(i64),

    // separators
    LeftParen,
    RightParen,
    OpenBrace,
    CloseBrace,
    Comma,
    Colon,
    SemiColon,
    Arrow,

    // operators
    Plus,
    Minus,
    Asterisk,
    Slash,
    Equals,
    Ampersand,
    Pipe,
    Caret,
    DoubleAsterisk,
    Percent,
    Tilde,
    GreaterThan,
    LessThan,
    GreaterThanEquals,
    LessThanEquals,
    EqualsEquals,
    BangEquals,

    // keywords
    Let,
    If,
    Else,
    True,
    False,
    While,
    Func,
    Return,

    // special
    Identifier,
    Whitespace,
    Bad,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: TextSpan,
}

impl Token {
    pub fn new(kind: TokenKind, span: TextSpan) -> Self {
        Self { kind, span }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Number(_) => write!(f, "Number"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Asterisk => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::OpenBrace => write!(f, "{{"),
            TokenKind::CloseBrace => write!(f, "}}"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Equals => write!(f, "="),
            TokenKind::Ampersand => write!(f, "&"),
            TokenKind::Pipe => write!(f, "|"),
            TokenKind::Caret => write!(f, "^"),
            TokenKind::DoubleAsterisk => write!(f, "**"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Tilde => write!(f, "~"),
            TokenKind::GreaterThan => write!(f, ">"),
            TokenKind::LessThan => write!(f, "<"),
            TokenKind::GreaterThanEquals => write!(f, ">="),
            TokenKind::LessThanEquals => write!(f, "<="),
            TokenKind::EqualsEquals => write!(f, "=="),
            TokenKind::BangEquals => write!(f, "!="),
            TokenKind::Let => write!(f, "let"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Func => write!(f, "func"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Bad => write!(f, "Bad"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    current_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            current_pos: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.current_pos >= self.input.len() {
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(self.current_pos, self.current_pos, "\0".into()),
            ));
        }

        let c = self.current_char()?;
        let start = self.current_pos;

        let kind = if Self::is_number_start(&c) {
            let number = self.consume_number();
            TokenKind::Number(number)
        } else if Self::is_whitespace(&c) {
            self.consume();
            TokenKind::Whitespace
        } else if Self::is_identifier_start(&c) {
            let identifier = self.consume_identifier();
            match identifier.as_str() {
                "let" => TokenKind::Let,
                "if" => TokenKind::If,
                "else" => TokenKind::Else,
                "true" => TokenKind::True,
                "false" => TokenKind::False,
                "while" => TokenKind::While,
                "func" => TokenKind::Func,
                "return" => TokenKind::Return,
                _ => TokenKind::Identifier,
            }
        } else {
            self.consume_punctuation()
        };

        let end = self.current_pos;
        let literal = self.input[start..end].to_string();
        let span = TextSpan::new(start, end, literal);
        Some(Token::new(kind, span))
    }
    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => self.lex_double_char('-', TokenKind::Minus, TokenKind::Arrow),
            '*' => self.lex_double_char('*', TokenKind::Asterisk, TokenKind::DoubleAsterisk),
            '%' => TokenKind::Percent,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '=' => self.lex_double_char('=', TokenKind::Equals, TokenKind::EqualsEquals),
            '&' => TokenKind::Ampersand,
            '|' => TokenKind::Pipe,
            '^' => TokenKind::Caret,
            '~' => TokenKind::Tilde,
            '>' => self.lex_double_char('=', TokenKind::GreaterThan, TokenKind::GreaterThanEquals),
            '<' => self.lex_double_char('=', TokenKind::LessThan, TokenKind::LessThanEquals),
            '!' => self.lex_double_char('=', TokenKind::Bad, TokenKind::BangEquals),
            '{' => TokenKind::OpenBrace,
            '}' => TokenKind::CloseBrace,
            ',' => TokenKind::Comma,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            _ => TokenKind::Bad,
        }
    }

    fn lex_double_char(
        &mut self,
        expected: char,
        one_char: TokenKind,
        double_char: TokenKind,
    ) -> TokenKind {
        if let Some(next) = self.current_char() {
            if next == expected {
                self.consume();
                double_char
            } else {
                one_char
            }
        } else {
            one_char
        }
    }

    fn is_number_start(c: &char) -> bool {
        c.is_ascii_digit()
    }

    fn is_identifier_start(c: &char) -> bool {
        c.is_alphabetic() || *c == '_'
    }

    fn is_whitespace(c: &char) -> bool {
        c.is_whitespace()
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }

    fn consume(&mut self) -> Option<char> {
        if self.current_pos >= self.input.len() {
            None
        } else {
            let c = self.current_char();
            self.current_pos += 1;
            c
        }
    }

    fn consume_identifier(&mut self) -> String {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if Self::is_identifier_start(&c) {
                self.consume();
                identifier.push(c);
            } else {
                break;
            }
        }
        identifier
    }

    fn consume_number(&mut self) -> i64 {
        let mut number: i64 = 0;
        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.consume();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            } else {
                break;
            }
        }
        number
    }
}
