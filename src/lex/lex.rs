use crate::token;

pub fn lex(s: String) -> Vec<token::Token> {
    let mut tokens: Vec<token::Token> = Vec::new();
    for c in s.chars() {
        match c {
            ',' => tokens.push(token::Token::Comma),
            ';' => tokens.push(token::Token::SemiColon),
            ':' => tokens.push(token::Token::Colon),
            '.' => tokens.push(token::Token::Dot),
            '(' => tokens.push(token::Token::LParen),
            ')' => tokens.push(token::Token::RParen),
            '}' => tokens.push(token::Token::RBrace),
            '{' => tokens.push(token::Token::LBrace),
            '=' => tokens.push(token::Token::Assign),
            _ => tokens.push(token::Token::Equal),
        }
    }
    println!("called lex::lex");
    tokens
}
