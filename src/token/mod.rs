use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    EOF,
    LBrace,
    RBrace,
    LParen,
    RParen,
    Comma,
    SemiColon,
    Colon,
    Dot,
    Assign,
    Equal,
}

pub fn to_string(tok: Token) -> &'static str {
    match tok {
        Token::EOF => "\0",
        Token::LBrace => "{",
        Token::RBrace => "}",
        Token::LParen => "(",
        Token::RParen => ")",
        Token::Comma => ",",
        Token::SemiColon => ";",
        Token::Colon => ":",
        Token::Dot => ".",
        Token::Assign => "=",
        Token::Equal => "==",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_to_string() {
        assert_eq!("}", to_string(Token::RBrace));
        assert_eq!("{", to_string(Token::LBrace));
        assert_eq!("(", to_string(Token::LParen));
        assert_eq!(")", to_string(Token::RParen));
        assert_eq!(",", to_string(Token::Comma));
        assert_eq!(";", to_string(Token::SemiColon));
        assert_eq!(":", to_string(Token::Colon));
        assert_eq!(".", to_string(Token::Dot));
        assert_eq!("=", to_string(Token::Assign));
        assert_eq!("==", to_string(Token::Equal));
    }
}
