use crate::token::Token;
use std::str::Chars;

pub struct Lexer<'a> {
    input: Chars<'a>,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input: input.chars(),
            current: None,
        };
        lexer.advance(); // Initialize the lexer with the first character
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.current {
            println!("Current character: {:?}", c); // Debugging the current character

            match c {
                ' ' => {
                    self.advance(); // Ensure lexer advances after parsing space
                    return Some(Token::Push(1)); // Placeholder for space token
                }
                '\t' => {
                    self.advance(); // Ensure lexer advances after parsing tab
                    return Some(Token::Push(2)); // Placeholder for tab token
                }
                '\n' => {
                    self.advance(); // Ensure lexer advances after parsing newline
                    return Some(Token::End); // Placeholder for newline token (End)
                }
                _ => {
                    println!("Unrecognized character: {:?}", c); // Debugging unhandled characters
                    self.advance(); // Ensure lexer advances even for unrecognized characters
                }
            }
        }
        None
    }

    // Advance to the next character in the input
    pub fn advance(&mut self) {
        self.current = self.input.next();
        println!("Advancing to: {:?}", self.current); // Debugging advancement
    }
}
