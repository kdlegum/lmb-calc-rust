use crate::tokeniser::Token;
use crate::abstract_tree::*;

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

fn token_to_op(token: &Token) -> Option<Op> {
    match token {
        Token::Plus => Some(Op::Add),
        Token::Minus => Some(Op::Sub),
        Token::AssignEquals => Some(Op::Is),
        Token::Equality => Some(Op::Eq),
        Token::Multiply => Some(Op::Mul),
        Token::Divide => Some(Op::Div),
        Token::Mod => Some(Op::Mod),
        _ => None
    }
}  

impl Parser {
    fn new(code: Vec<Token>) -> Parser {
        Parser {tokens:code, cursor:0}
    } 
    fn peek(&self) -> &Token { &self.tokens[self.cursor] }

    fn next(&mut self) -> &Token { self.cursor += 1; &self.tokens[self.cursor-1]}
    
    fn parse_program(&mut self) -> Program {
        let mut stmts: Program = vec![];
        loop {
            match self.parse_statement() {
                Ok(Statement::EOP) => break,
                Ok(s) => stmts.push(s),
                Err(e) => panic!("{}", e),
            }
        }
        stmts
    }

    fn parse_statement(&mut self) -> Result<Statement, String> {
        match self.peek() {
            Token::Let => {
                self.next();
                let Token::Ident(s) = self.next() else {return Err("Token after let not an ident".to_string())};
                let new_var_str = s.clone();
                if *self.next() != Token::AssignEquals {return Err("Let used incorrectly".to_string())};
                let expr = self.parse_expr()?;
                if *self.next() != Token::Semicolon {return Err("Expected semicolon after let statement".to_string())};
                return Ok(Statement::Let(new_var_str, expr));
            }
            Token::Return => {
                self.next();
                let expr = self.parse_expr()?;
                if *self.next() != Token::Semicolon {return Err("Expected semicolon after return statement".to_string())};
                return Ok(Statement::Return(expr));
            }
            Token::EOP => {
                return Ok(Statement::EOP);
            }
            _ => return Err("Not a valid statement".to_string()),
        }
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut lhs = self.parse_atom()?;
        while *self.peek() == Token::Multiply || *self.peek() == Token::Mod || *self.peek() == Token::Divide {
            let operator = token_to_op(self.next()).ok_or("Invalid operation for parse_term")?;
            let rhs = Box::new(self.parse_atom()?);
            lhs = Expr::BinaryOp(Box::new(lhs), operator, rhs);
        }
        Ok(lhs)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        // Handle lower level +, -, call parse_term when needed since they need to be done first
        let mut lhs = self.parse_term()?;
        while *self.peek() == Token::Plus || *self.peek() == Token::Minus {
            let operator = token_to_op(self.next()).ok_or("Invalid operator for parse_expr")?;
            let rhs = Box::new(self.parse_term()?);
            lhs = Expr::BinaryOp(Box::new(lhs), operator, rhs);
        }
        Ok(lhs)

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

#[cfg(test)]
mod tests {
    use super::*;

    // === parse_atom tests ===

    #[test]
    fn atom_number() {
        let mut p = Parser::new(vec![Token::Number(42), Token::Semicolon]);
        assert_eq!(p.parse_atom().unwrap(), Expr::Number(42));
    }

    #[test]
    fn atom_ident() {
        let mut p = Parser::new(vec![Token::Ident("x".to_string()), Token::Semicolon]);
        assert_eq!(p.parse_atom().unwrap(), Expr::Ident("x".to_string()));
    }

    #[test]
    fn atom_bool() {
        let mut p = Parser::new(vec![Token::Boolean(true), Token::Semicolon]);
        assert_eq!(p.parse_atom().unwrap(), Expr::Bool(true));
    }

    #[test]
    fn atom_parenthesised_expr() {
        // (1 + 2)
        let mut p = Parser::new(vec![
            Token::LParen, Token::Number(1), Token::Plus, Token::Number(2), Token::RParen,
            Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_atom().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(1)), Op::Add, Box::new(Expr::Number(2)))
        );
    }

    #[test]
    fn atom_unclosed_paren() {
        // (1 + 2  -- missing RParen
        let mut p = Parser::new(vec![
            Token::LParen, Token::Number(1), Token::Plus, Token::Number(2), Token::Semicolon,
        ]);
        assert!(p.parse_atom().is_err());
    }

    #[test]
    fn atom_invalid_token() {
        let mut p = Parser::new(vec![Token::Plus, Token::Semicolon]);
        assert!(p.parse_atom().is_err());
    }

    // === parse_term tests ===

    #[test]
    fn term_single_atom() {
        let mut p = Parser::new(vec![Token::Number(5), Token::Semicolon]);
        assert_eq!(p.parse_term().unwrap(), Expr::Number(5));
    }

    #[test]
    fn term_multiply() {
        // 3 * 4
        let mut p = Parser::new(vec![
            Token::Number(3), Token::Multiply, Token::Number(4), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_term().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(3)), Op::Mul, Box::new(Expr::Number(4)))
        );
    }

    #[test]
    fn term_divide() {
        // 10 / 2
        let mut p = Parser::new(vec![
            Token::Number(10), Token::Divide, Token::Number(2), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_term().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(10)), Op::Div, Box::new(Expr::Number(2)))
        );
    }

    #[test]
    fn term_mod() {
        // 7 % 3
        let mut p = Parser::new(vec![
            Token::Number(7), Token::Mod, Token::Number(3), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_term().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(7)), Op::Mod, Box::new(Expr::Number(3)))
        );
    }

    #[test]
    fn term_chained_left_associative() {
        // 2 * 3 / 4  =>  (2 * 3) / 4
        let mut p = Parser::new(vec![
            Token::Number(2), Token::Multiply, Token::Number(3),
            Token::Divide, Token::Number(4), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_term().unwrap(),
            Expr::BinaryOp(
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Number(2)), Op::Mul, Box::new(Expr::Number(3))
                )),
                Op::Div,
                Box::new(Expr::Number(4))
            )
        );
    }

    // === parse_expr tests ===

    #[test]
    fn expr_single_number() {
        let mut p = Parser::new(vec![Token::Number(9), Token::Semicolon]);
        assert_eq!(p.parse_expr().unwrap(), Expr::Number(9));
    }

    #[test]
    fn expr_addition() {
        // 1 + 2
        let mut p = Parser::new(vec![
            Token::Number(1), Token::Plus, Token::Number(2), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_expr().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(1)), Op::Add, Box::new(Expr::Number(2)))
        );
    }

    #[test]
    fn expr_subtraction() {
        // 5 - 3
        let mut p = Parser::new(vec![
            Token::Number(5), Token::Minus, Token::Number(3), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_expr().unwrap(),
            Expr::BinaryOp(Box::new(Expr::Number(5)), Op::Sub, Box::new(Expr::Number(3)))
        );
    }

    #[test]
    fn expr_precedence_mul_before_add() {
        // 1 + 2 * 3  =>  1 + (2 * 3)
        let mut p = Parser::new(vec![
            Token::Number(1), Token::Plus,
            Token::Number(2), Token::Multiply, Token::Number(3),
            Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_expr().unwrap(),
            Expr::BinaryOp(
                Box::new(Expr::Number(1)),
                Op::Add,
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Number(2)), Op::Mul, Box::new(Expr::Number(3))
                ))
            )
        );
    }

    #[test]
    fn expr_parens_override_precedence() {
        // (1 + 2) * 3  =>  (1 + 2) * 3
        let mut p = Parser::new(vec![
            Token::LParen, Token::Number(1), Token::Plus, Token::Number(2), Token::RParen,
            Token::Multiply, Token::Number(3),
            Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_expr().unwrap(),
            Expr::BinaryOp(
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Number(1)), Op::Add, Box::new(Expr::Number(2))
                )),
                Op::Mul,
                Box::new(Expr::Number(3))
            )
        );
    }

    #[test]
    fn expr_chained_addition() {
        // 1 + 2 + 3  =>  (1 + 2) + 3
        let mut p = Parser::new(vec![
            Token::Number(1), Token::Plus, Token::Number(2),
            Token::Plus, Token::Number(3), Token::Semicolon,
        ]);
        assert_eq!(
            p.parse_expr().unwrap(),
            Expr::BinaryOp(
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Number(1)), Op::Add, Box::new(Expr::Number(2))
                )),
                Op::Add,
                Box::new(Expr::Number(3))
            )
        );
    }

    // === parse_program tests ===

    #[test]
    fn program_empty() {
        // Just EOP — should produce an empty program
        let mut p = Parser::new(vec![Token::EOP]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 0);
    }

    #[test]
    fn program_single_let() {
        // let x = 5;
        let mut p = Parser::new(vec![
            Token::Let, Token::Ident("x".to_string()), Token::AssignEquals,
            Token::Number(5), Token::Semicolon,
            Token::EOP,
        ]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 1);
        assert_eq!(prog[0], Statement::Let("x".to_string(), Expr::Number(5)));
    }

    #[test]
    fn program_single_return() {
        // return 42;
        let mut p = Parser::new(vec![
            Token::Return, Token::Number(42), Token::Semicolon,
            Token::EOP,
        ]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 1);
        assert_eq!(prog[0], Statement::Return(Expr::Number(42)));
    }

    #[test]
    fn program_let_then_return() {
        // let x = 10
        // return x + 1
        let mut p = Parser::new(vec![
            Token::Let, Token::Ident("x".to_string()), Token::AssignEquals, Token::Number(10), Token::Semicolon,
            Token::Return, Token::Ident("x".to_string()), Token::Plus, Token::Number(1), Token::Semicolon,
            Token::EOP,
        ]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 2);
        assert_eq!(prog[0], Statement::Let("x".to_string(), Expr::Number(10)));
        assert_eq!(
            prog[1],
            Statement::Return(Expr::BinaryOp(
                Box::new(Expr::Ident("x".to_string())),
                Op::Add,
                Box::new(Expr::Number(1)),
            ))
        );
    }

    #[test]
    fn program_multiple_lets() {
        // let a = 1
        // let b = 2
        // let c = a + b
        let mut p = Parser::new(vec![
            Token::Let, Token::Ident("a".to_string()), Token::AssignEquals, Token::Number(1), Token::Semicolon,
            Token::Let, Token::Ident("b".to_string()), Token::AssignEquals, Token::Number(2), Token::Semicolon,
            Token::Let, Token::Ident("c".to_string()), Token::AssignEquals,
                Token::Ident("a".to_string()), Token::Plus, Token::Ident("b".to_string()), Token::Semicolon,
            Token::EOP,
        ]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 3);
        assert_eq!(prog[0], Statement::Let("a".to_string(), Expr::Number(1)));
        assert_eq!(prog[1], Statement::Let("b".to_string(), Expr::Number(2)));
        assert_eq!(
            prog[2],
            Statement::Let("c".to_string(), Expr::BinaryOp(
                Box::new(Expr::Ident("a".to_string())),
                Op::Add,
                Box::new(Expr::Ident("b".to_string())),
            ))
        );
    }

    #[test]
    fn program_return_complex_expr() {
        // return (1 + 2) * 3
        let mut p = Parser::new(vec![
            Token::Return,
            Token::LParen, Token::Number(1), Token::Plus, Token::Number(2), Token::RParen,
            Token::Multiply, Token::Number(3), Token::Semicolon,
            Token::EOP,
        ]);
        let prog = p.parse_program();
        assert_eq!(prog.len(), 1);
        assert_eq!(
            prog[0],
            Statement::Return(Expr::BinaryOp(
                Box::new(Expr::BinaryOp(
                    Box::new(Expr::Number(1)), Op::Add, Box::new(Expr::Number(2))
                )),
                Op::Mul,
                Box::new(Expr::Number(3)),
            ))
        );
    }

    #[test]
    #[should_panic]
    fn program_invalid_statement() {
        // Starting with a number — not a valid statement
        let mut p = Parser::new(vec![Token::Number(5), Token::EOP]);
        p.parse_program();
    }

    #[test]
    #[should_panic]
    fn program_let_missing_ident() {
        // let = 5  — missing identifier
        let mut p = Parser::new(vec![
            Token::Let, Token::AssignEquals, Token::Number(5), Token::EOP,
        ]);
        p.parse_program();
    }

    #[test]
    #[should_panic]
    fn program_let_missing_equals() {
        // let x 5  — missing =
        let mut p = Parser::new(vec![
            Token::Let, Token::Ident("x".to_string()), Token::Number(5), Token::EOP,
        ]);
        p.parse_program();
    }
}