mod token {
    pub type TokenType = &'static str;

    pub struct Token {
        pub token_type: TokenType,
        pub literal: String,
    }

    impl Token {
        pub fn new(token_type: TokenType, ch: char) -> Token {
            Token {
                token_type,
                literal: ch.to_string(),
            }
        }
    }

    pub const ILLEGAL: &str = "ILLEGAL";
    pub const EOF: &str = "EOF";

    pub const IDENT: &str = "IDENT";
    pub const INT: &str = "INT";

    pub const ASSIGN: &str = "ASSIGN";
    pub const PLUS: &str = "PLUS";

    pub const COMMA: &str = "COMMA";
    pub const SEMICOLON: &str = "SEMICOLON";

    pub const LPAREN: &str = "LPAREN";
    pub const RPAREN: &str = "RPAREN";
    pub const LBRACE: &str = "LBRACE";
    pub const RBRACE: &str = "RBRACE";

    pub const FUNCTION: &str = "FUNCTION";
    pub const LET: &str = "LET";
}

mod lexer {

    use crate::token;

    struct Lexer {
        input: String,
        position: usize,
        read_position: usize,
        ch: Option<char>,
    }

    impl Lexer {
        pub fn new(input: String) -> Lexer {
            let mut l = Lexer {
                input,
                position: 0,
                read_position: 0,
                ch: None,
            };
            l.read_char();
            l
        }

        pub fn next_token(&mut self) -> token::Token {
            let ch = self.ch.unwrap_or('\0');

            let token = match ch {
                '=' => token::Token::new(token::ASSIGN, ch),
                ';' => token::Token::new(token::SEMICOLON, ch),
                '(' => token::Token::new(token::LPAREN, ch),
                ')' => token::Token::new(token::RPAREN, ch),
                ',' => token::Token::new(token::COMMA, ch),
                '+' => token::Token::new(token::PLUS, ch),
                '{' => token::Token::new(token::LBRACE, ch),
                '}' => token::Token::new(token::RBRACE, ch),
                '\0' => token::Token {
                    literal: String::from(""),
                    token_type: token::EOF,
                },
                _ => panic!("Invalid input"),
            };

            self.read_char();
            token
        }

        pub fn read_char(&mut self) {
            self.ch = self.input.chars().nth(self.read_position);
            self.position = self.read_position;
            self.read_position += 1;
        }
    }

    #[cfg(test)]
    mod tests {

        use crate::token;

        fn test_next_token(input: String, test: &[token::Token]) {
            let mut l = super::Lexer::new(input);

            for elem in test.iter() {
                let tok = l.next_token();

                assert_eq!(tok.token_type, elem.token_type);
                assert_eq!(tok.literal, elem.literal);
            }
        }

        #[test]
        fn test_next_token_simple() {
            let input = String::from("=+(){},;");

            let test: [token::Token; 9] = [
                token::Token::new(token::ASSIGN, '='),
                token::Token::new(token::PLUS, '+'),
                token::Token::new(token::LPAREN, '('),
                token::Token::new(token::RPAREN, ')'),
                token::Token::new(token::LBRACE, '{'),
                token::Token::new(token::RBRACE, '}'),
                token::Token::new(token::COMMA, ','),
                token::Token::new(token::SEMICOLON, ';'),
                token::Token {
                    literal: String::from(""),
                    token_type: token::EOF,
                },
            ];

            test_next_token(input, &test);
        }

        #[test]
        fn test_next_token_complex() {
            let input = String::from(
                "
                let five = 5;
                let ten = 10;
                let add = fn(x, y) {
                    x + y;
                };

                let result = add(five, ten);
            ",
            );

            let test: [token::Token; 37] = [
                token::Token {
                    literal: String::from("let"),
                    token_type: token::LET,
                },
                token::Token {
                    literal: String::from("five"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("="),
                    token_type: token::ASSIGN,
                },
                token::Token {
                    literal: String::from("5"),
                    token_type: token::INT,
                },
                token::Token {
                    literal: String::from(";"),
                    token_type: token::SEMICOLON,
                },
                token::Token {
                    literal: String::from("let"),
                    token_type: token::LET,
                },
                token::Token {
                    literal: String::from("ten"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("="),
                    token_type: token::ASSIGN,
                },
                token::Token {
                    literal: String::from("10"),
                    token_type: token::INT,
                },
                token::Token {
                    literal: String::from(";"),
                    token_type: token::SEMICOLON,
                },
                token::Token {
                    literal: String::from("let"),
                    token_type: token::LET,
                },
                token::Token {
                    literal: String::from("add"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("="),
                    token_type: token::ASSIGN,
                },
                token::Token {
                    literal: String::from("fn"),
                    token_type: token::FUNCTION,
                },
                token::Token {
                    literal: String::from("("),
                    token_type: token::LPAREN,
                },
                token::Token {
                    literal: String::from("x"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from(","),
                    token_type: token::COMMA,
                },
                token::Token {
                    literal: String::from("y"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from(")"),
                    token_type: token::RPAREN,
                },
                token::Token {
                    literal: String::from("{"),
                    token_type: token::LBRACE,
                },
                token::Token {
                    literal: String::from("x"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("+"),
                    token_type: token::PLUS,
                },
                token::Token {
                    literal: String::from("y"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from(";"),
                    token_type: token::SEMICOLON,
                },
                token::Token {
                    literal: String::from("}"),
                    token_type: token::RBRACE,
                },
                token::Token {
                    literal: String::from(";"),
                    token_type: token::SEMICOLON,
                },
                token::Token {
                    literal: String::from("let"),
                    token_type: token::LET,
                },
                token::Token {
                    literal: String::from("result"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("="),
                    token_type: token::ASSIGN,
                },
                token::Token {
                    literal: String::from("add"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from("("),
                    token_type: token::LPAREN,
                },
                token::Token {
                    literal: String::from("five"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from(","),
                    token_type: token::COMMA,
                },
                token::Token {
                    literal: String::from("ten"),
                    token_type: token::IDENT,
                },
                token::Token {
                    literal: String::from(")"),
                    token_type: token::RPAREN,
                },
                token::Token {
                    literal: String::from(";"),
                    token_type: token::SEMICOLON,
                },
                token::Token {
                    literal: String::from(""),
                    token_type: token::EOF,
                },
            ];

            test_next_token(input, &test);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
