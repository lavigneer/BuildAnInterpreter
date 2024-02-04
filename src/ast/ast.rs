use crate::lexer::lexer::Lexer;
use crate::lexer::lexer::Token;

pub enum Statement {
    Let(LetStatment),
}
pub enum Expression {}

pub struct Program {
    statements: Vec<Statement>,
}

pub struct LetStatment {
    name: Identifier,
    value: Expression,
}

pub struct Identifier {
    value: String,
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Option<Token>,
    peek_token: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(lexer: &'a mut Lexer) -> Self {
        let mut parser = Parser {
            lexer,
            current_token: None,
            peek_token: None,
        };

        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current_token = std::mem::replace(&mut self.peek_token, self.lexer.next_token().ok());
    }

    fn parse_program(&mut self) -> Program {
        todo!()
    }
}
