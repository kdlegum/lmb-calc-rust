use crate::tokeniser::Token;
use crate::abstract_tree::*;

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    fn new(code: Vec<Token>) -> Parser {
        Parser {tokens:code, cursor:0}
    } 
    fn peek(&self) -> &Token { &self.tokens[self.cursor] }

    fn next(&mut self) -> &Token { self.cursor += 1; &self.tokens[self.cursor-1]}
    
    fn parse_program(&mut self) -> Program {
     todo!()   
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {

        todo!()
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        // Handle high precedence *, /, %
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        // Handle lower level +, -, call parse_term when needed since they need to be done first
        loop {
            let component_collection: 
            match self.peek() {
                Token::Semicolon => {self.next(); break;}

            }
        }
    }

    fn parse_atom(&mut self) -> Result<Expr, String> {
        // Handle single tokens, and also reversing precedence due to brackets
        let token = self.next();
        match token {
            Token::Number(num) => Ok(Expr::Number(*num)),
            Token::Ident(str) => Ok(Expr::Ident(str.clone())),
            Token::Boolean(bool) => Ok(Expr::Bool(*bool)),
            Token::LParen => {
                let result = self.parse_expr()?;
                let current = self.next();
                if *current == Token::RParen { Ok(result) }
                else { Err("Unclosed parentheses".to_string()) }

            },
            _ => Err("Invalid input to parse_atom".to_string())
        }

    }
}