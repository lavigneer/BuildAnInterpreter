use std::fmt::Display;

use anyhow::Result;

#[derive(PartialEq, Debug)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Int(String),

    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,

    Lt,
    Gt,
    Eq,
    NotEq,

    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(x) => write!(f, "Ident({})", x),
            Token::Int(x) => write!(f, "Int({})", x),
            Token::Illegal => write!(f, "Illegal"),
            Token::Eof => write!(f, "Eof"),
            Token::Assign => write!(f, "Assign"),
            Token::Bang => write!(f, "Bang"),
            Token::Minus => write!(f, "Minus"),
            Token::Slash => write!(f, "Slash"),
            Token::Asterisk => write!(f, "Asterisk"),
            Token::Eq => write!(f, "Eq"),
            Token::NotEq => write!(f, "NotEq "),
            Token::Lt => write!(f, "Lt "),
            Token::Gt => write!(f, "Gt"),
            Token::Plus => write!(f, "Plus"),
            Token::Comma => write!(f, "Comma"),
            Token::Semicolon => write!(f, "Semicolon"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::LBrace => write!(f, "LBrace "),
            Token::RBrace => write!(f, "RBrace"),
            Token::Function => write!(f, "Function"),
            Token::Let => write!(f, "Let"),
            Token::If => write!(f, "If"),
            Token::Else => write!(f, "Else"),
            Token::Return => write!(f, "Return"),
            Token::True => write!(f, "True"),
            Token::False => write!(f, "False"),
        }
    }
}

pub struct Lexer {
    input: Vec<u8>,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input: input.into_bytes(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let token = match self.ch {
            b'=' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::Eq
                }
                _ => Token::Assign,
            },
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::Bang,
            },
            b'/' => Token::Slash,
            b'*' => Token::Asterisk,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b',' => Token::Comma,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LBrace,
            b'}' => Token::RBrace,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_identifier();
                return Ok(match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "true" => Token::True,
                    "false" => Token::False,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(ident),
                });
            }
            b'0'..=b'9' => return Ok(Token::Int(self.read_number())),
            0 => Token::Eof,
            _ => Token::Illegal,
        };

        self.read_char();
        Ok(token)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            0
        } else {
            self.input[self.read_position]
        }
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }

        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char();
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_next_token_simple() {
        let input = String::from("=+(){},;");

        let mut lexer = Lexer::new(input);

        let test = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::Eof,
        ];

        for elem in test {
            let tok = lexer.next_token().unwrap();
            assert_eq!(tok, elem);
        }
    }

    #[test]
    fn test_next_token_complex() {
        let input = r#"let five = 5 ;
            let ten = 10;

            let add = fn(x, y) {
                x + y;
            };

            let result = add(five, ten);
            !-/*5;
            5 < 10 > 5;

            if (5 < 10) {
                return true;
            } else {
                return false;
            }

            10 == 10;
            10 != 9;
        "#;

        let mut lexer = Lexer::new(input.into());

        let test = vec![
            Token::Let,
            Token::Ident(String::from("five")),
            Token::Assign,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("ten")),
            Token::Assign,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident(String::from("x")),
            Token::Comma,
            Token::Ident(String::from("y")),
            Token::RParen,
            Token::LBrace,
            Token::Ident(String::from("x")),
            Token::Plus,
            Token::Ident(String::from("y")),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident(String::from("result")),
            Token::Assign,
            Token::Ident(String::from("add")),
            Token::LParen,
            Token::Ident(String::from("five")),
            Token::Comma,
            Token::Ident(String::from("ten")),
            Token::RParen,
            Token::Semicolon,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::Int(String::from("5")),
            Token::Lt,
            Token::Int(String::from("10")),
            Token::Gt,
            Token::Int(String::from("5")),
            Token::Semicolon,
            Token::If,
            Token::LParen,
            Token::Int(String::from("5")),
            Token::Lt,
            Token::Int(String::from("10")),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RBrace,
            Token::Int(String::from("10")),
            Token::Eq,
            Token::Int(String::from("10")),
            Token::Semicolon,
            Token::Int(String::from("10")),
            Token::NotEq,
            Token::Int(String::from("9")),
            Token::Semicolon,
            Token::Eof,
        ];

        for elem in test {
            let tok = lexer.next_token().unwrap();
            println!("{:?} | {:?}", tok, elem);
            assert_eq!(tok, elem);
        }
    }
}
