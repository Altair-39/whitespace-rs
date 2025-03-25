use crate::token::Token;

pub struct Interpreter {
    stack: Vec<i64>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { stack: Vec::new() }
    }

    pub fn execute(&mut self, program: &[Token]) {
        let mut i = 0;
        while i < program.len() {
            match &program[i] {
                Token::Push(n) => {
                    self.stack.push(*n);
                    println!("Pushed: {}", n);
                }
                Token::Add => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a + b);
                        println!("Add: Result is {}", a + b);
                    }
                }
                Token::Subtract => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a - b);
                        println!("Subtract: Result is {}", a - b);
                    }
                }
                Token::Multiply => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        self.stack.push(a * b);
                        println!("Multiply: Result is {}", a * b);
                    }
                }
                Token::Divide => {
                    if self.stack.len() >= 2 {
                        let b = self.stack.pop().unwrap();
                        let a = self.stack.pop().unwrap();
                        if b != 0 {
                            self.stack.push(a / b);
                            println!("Divide: Result is {}", a / b);
                        } else {
                            println!("Error: Division by zero");
                        }
                    }
                }
                Token::OutputChar => {
                    if let Some(value) = self.stack.pop() {
                        print!("{}", value as u8 as char);
                    }
                }
                Token::End => {
                    println!("End of program");
                    break;
                }
                _ => {}
            }

            i += 1;
        }
    }
}
