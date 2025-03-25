use crate::lexer::Lexer;
use crate::token::Token;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        let mut program = Vec::new();

        while let Some(token) = self.lexer.next_token() {
            match token {
                Token::Push(n) => program.push(Token::Push(n)),
                Token::Add => program.push(Token::Add),
                Token::Subtract => program.push(Token::Subtract),
                Token::Multiply => program.push(Token::Multiply),
                Token::Divide => program.push(Token::Divide),
                Token::Modulo => program.push(Token::Modulo),
                Token::Store => program.push(Token::Store),
                Token::Retrieve => program.push(Token::Retrieve),
                Token::Label(name) => program.push(Token::Label(name)),
                Token::Call(name) => program.push(Token::Call(name)),
                Token::Jump(name) => program.push(Token::Jump(name)),
                Token::JumpIfZero(name) => program.push(Token::JumpIfZero(name)),
                Token::JumpIfNegative(name) => program.push(Token::JumpIfNegative(name)),
                Token::Return => program.push(Token::Return),
                Token::End => program.push(Token::End),
                Token::OutputChar => program.push(Token::OutputChar),
                Token::OutputNumber => program.push(Token::OutputNumber),
                Token::ReadChar => program.push(Token::ReadChar),
                Token::ReadNumber => program.push(Token::ReadNumber),
                _ => {}
            }
        }

        program
    }
}
