// let x = 5 + 3
#[derive(Debug, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Return,
    Is,
    // Might wanna change the name of fn
    Fn,

    // Literals
    Number(i32),
    Boolean(bool),

    Ident(String),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Mod,
    AssignEquals,
    Equality,

    // Punctuation
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Arrow,

    EOP,
}

impl Token {
    fn is_operator(&self) -> bool {
        matches!(self, Token::Plus | Token::Minus | Token::Multiply | Token::Divide | Token::Mod | Token::AssignEquals | Token::Equality)
    }
}

fn match_keyword(word: &str) -> Token {
    match word {
                "let" => Token::Let,
                "return" => Token::Return,
                "is" => Token::Is,
                "fn" => Token::Fn,
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                "=>" => Token::Arrow,
                s => {
                    match s.parse::<i32>() {
                        Ok(num) => Token::Number(num),
                        Err(_) => Token::Ident(s.to_string()),
                    }
            }
        }
}

fn tokenise(code: &str) -> Vec<Token> {
    let key_chars = ['+', '-', '*', '/', '%', '(', ')', '{', '}', ';', '='];
    let mut chars = code.chars().peekable();
    let mut tokens: Vec<Token> = Vec::new();
    while let Some(&c) = chars.peek() {
        match c {
            '+' => { chars.next(); tokens.push(Token::Plus); }
            '-' => { chars.next(); tokens.push(Token::Minus); }
            '*' => { chars.next(); tokens.push(Token::Multiply); }
            '/' => { chars.next(); tokens.push(Token::Divide); }
            '%' => { chars.next(); tokens.push(Token::Mod); }
            '(' => { chars.next(); tokens.push(Token::LParen); }
            ')' => { chars.next(); tokens.push(Token::RParen); }
            '{' => { chars.next(); tokens.push(Token::LBrace); }
            '}' => { chars.next(); tokens.push(Token::RBrace); }
            ';' => { chars.next(); tokens.push(Token::Semicolon); }
            '=' => {
                chars.next();
                if chars.peek() == Some(&'=') {
                    chars.next();
                    tokens.push(Token::Equality);
                } else if chars.peek() == Some(&'>') {
                    chars.next();
                    tokens.push(Token::Arrow);
                } else {
                    tokens.push(Token::AssignEquals);
                }
            }
            a => { 
                if a.is_whitespace() {
                    chars.next();
                } else {
                    let mut char_collection: String = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_whitespace() || key_chars.contains(&d) { break; }
                        chars.next();
                        char_collection.push(d);
                    }
                    tokens.push(match_keyword(&char_collection));
                }
            }
        }
    }
    tokens.push(Token::EOP);
    tokens
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenise_assignment() {
        let test = "let x = 5;";
        let result = tokenise(test);
        assert_eq!(result, vec![Token::Let, Token::Ident("x".to_string()), Token::AssignEquals, Token::Number(5), Token::Semicolon, Token::EOP])
    }

    #[test]
    fn tokenise_multiple_lines_example() {
        let test = "let x = 5;
let y = 6;
return x + y is 11;";
        let result = tokenise(test);
        assert_eq!(result, vec![Token::Let, Token::Ident("x".to_string()), Token::AssignEquals, Token::Number(5), Token::Semicolon,
        Token::Let, Token::Ident("y".to_string()), Token::AssignEquals, Token::Number(6), Token::Semicolon,
        Token::Return, Token::Ident("x".to_string()), Token::Plus, Token::Ident("y".to_string()), Token::Is, Token::Number(11), Token::Semicolon, Token::EOP]);
    }
}